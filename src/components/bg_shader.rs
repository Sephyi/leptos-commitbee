// SPDX-FileCopyrightText: 2026 Sephyi <me@sephy.io>
//
// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

//! Variant A — Shader flow field background.
//!
//! A full-viewport `<canvas>` rendered with a WebGL2 fragment shader that
//! computes curl-noise flow in a honey/amber/bark palette. Mouse position is
//! passed as a uniform so the field subtly warps around the cursor. Renders
//! into a single fullscreen triangle — no geometry beyond that.
//!
//! If WebGL2 is unavailable or the user prefers reduced motion, the canvas
//! element is hidden and the page falls through to the body background color.
//!
//! This is an `#[island]` that mounts at the App root, rendering on every page.

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

const RENDER_SCALE: f64 = 0.6;

const VERTEX_SHADER: &str = r#"#version 300 es
precision highp float;
layout(location = 0) in vec2 aPosition;
out vec2 vUv;
void main() {
    vUv = aPosition * 0.5 + 0.5;
    gl_Position = vec4(aPosition, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es
precision highp float;

in vec2 vUv;
out vec4 outColor;

uniform vec2 uResolution;
uniform float uTime;
uniform vec2 uMouse;
uniform float uDark;

// Simplex-ish hash + value noise. Compact, good enough for lava-lamp flow.
vec2 hash2(vec2 p) {
    p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
    return -1.0 + 2.0 * fract(sin(p) * 43758.5453123);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(dot(hash2(i + vec2(0.0, 0.0)), f - vec2(0.0, 0.0)),
            dot(hash2(i + vec2(1.0, 0.0)), f - vec2(1.0, 0.0)), u.x),
        mix(dot(hash2(i + vec2(0.0, 1.0)), f - vec2(0.0, 1.0)),
            dot(hash2(i + vec2(1.0, 1.0)), f - vec2(1.0, 1.0)), u.x),
        u.y);
}

// Fractal Brownian motion.
float fbm(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    mat2 rot = mat2(0.8, 0.6, -0.6, 0.8);
    for (int i = 0; i < 5; i++) {
        v += a * noise(p);
        p = rot * p * 2.0 + vec2(100.0);
        a *= 0.5;
    }
    return v;
}

void main() {
    vec2 uv = vUv;
    float aspect = uResolution.x / uResolution.y;
    vec2 p = vec2((uv.x - 0.5) * aspect, uv.y - 0.5);

    // Mouse warp: gentle gravitational pull around the cursor.
    vec2 m = vec2((uMouse.x - 0.5) * aspect, uMouse.y - 0.5);
    vec2 toMouse = p - m;
    float d = length(toMouse);
    p += toMouse * 0.08 * exp(-d * 6.0);

    // Two layers of fbm sliding at different speeds in different directions.
    float t = uTime * 0.04;
    vec2 q = vec2(
        fbm(p * 1.5 + vec2(t * 1.2, t * 0.7)),
        fbm(p * 1.5 + vec2(-t * 0.9, t * 1.3) + vec2(5.2, 1.3))
    );
    vec2 r = vec2(
        fbm(p * 1.5 + q * 2.0 + vec2(1.7, 9.2) + t * 0.5),
        fbm(p * 1.5 + q * 2.0 + vec2(8.3, 2.8) - t * 0.6)
    );
    float f = fbm(p * 1.5 + r * 1.5);
    f = smoothstep(-0.2, 1.0, f);

    // Bee palette — light mode tokens on top, dark mode mix below.
    vec3 honey      = vec3(0.960, 0.619, 0.043);
    vec3 honeyLight = vec3(0.988, 0.827, 0.301);
    vec3 bark       = vec3(0.110, 0.098, 0.090);
    vec3 cream      = vec3(1.000, 0.984, 0.921);

    vec3 col = mix(cream, honeyLight, f);
    col = mix(col, honey, smoothstep(0.5, 0.9, f));
    col = mix(col, bark, smoothstep(0.85, 1.1, length(r)) * 0.35);

    // Dark-mode inversion: flip the base toward dark, keep the honey highlights.
    vec3 darkBase = vec3(0.047, 0.039, 0.035);
    vec3 darkMid  = vec3(0.180, 0.090, 0.010);
    vec3 darkCol  = mix(darkBase, darkMid, f);
    darkCol = mix(darkCol, honey * 0.55, smoothstep(0.55, 0.95, f));

    col = mix(col, darkCol, uDark);

    // Vignette + tiny grain to break up banding.
    float vign = smoothstep(1.2, 0.3, length(p));
    col *= mix(0.75, 1.0, vign);
    col += (hash2(uv * uResolution).x) * 0.015;

    // Fully opaque — the hero headline is large, bold, and has enough weight
    // to stay legible over the flow field even at full saturation.
    outColor = vec4(col, 1.0);
}
"#;

fn compile_shader(gl: &GL, kind: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(kind)
        .ok_or_else(|| "unable to create shader".to_string())?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "shader compile error".to_string()))
    }
}

fn link_program(gl: &GL, vs: &WebGlShader, fs: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| "unable to create program".to_string())?;
    gl.attach_shader(&program, vs);
    gl.attach_shader(&program, fs);
    gl.link_program(&program);
    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "program link error".to_string()))
    }
}

#[island]
pub fn BgShader() -> impl IntoView {
    let node = NodeRef::<leptos::html::Canvas>::new();

    Effect::new(move |_| {
        let Some(canvas) = node.get() else {
            return;
        };
        let canvas: HtmlCanvasElement = canvas.unchecked_into();

        // Reduced-motion opt-out.
        if let Some(window) = web_sys::window()
            && let Ok(Some(mq)) = window.match_media("(prefers-reduced-motion: reduce)")
            && mq.matches()
        {
            return;
        }

        let Ok(Some(ctx)) = canvas.get_context("webgl2") else {
            return;
        };
        let gl: GL = match ctx.dyn_into() {
            Ok(g) => g,
            Err(_) => return,
        };

        let Ok(vs) = compile_shader(&gl, GL::VERTEX_SHADER, VERTEX_SHADER) else {
            return;
        };
        let Ok(fs) = compile_shader(&gl, GL::FRAGMENT_SHADER, FRAGMENT_SHADER) else {
            return;
        };
        let Ok(program) = link_program(&gl, &vs, &fs) else {
            return;
        };
        gl.use_program(Some(&program));

        // VAO + position buffer: fullscreen triangle in clip space [-1, 3].
        // Using a real attrib (instead of gl_VertexID with no VAO) keeps
        // Firefox/Mac off the emulation slow path and avoids the "vertex
        // attrib 0 not enabled" warning.
        let Some(vao) = gl.create_vertex_array() else {
            return;
        };
        gl.bind_vertex_array(Some(&vao));

        let Some(buffer) = gl.create_buffer() else {
            return;
        };
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
        let verts: [f32; 6] = [-1.0, -1.0, 3.0, -1.0, -1.0, 3.0];
        unsafe {
            let view = js_sys::Float32Array::view(&verts);
            gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &view, GL::STATIC_DRAW);
        }
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);

        let u_resolution = gl.get_uniform_location(&program, "uResolution");
        let u_time = gl.get_uniform_location(&program, "uTime");
        let u_mouse = gl.get_uniform_location(&program, "uMouse");
        let u_dark = gl.get_uniform_location(&program, "uDark");

        // Shared state across the rAF loop and the event listeners.
        let state = std::rc::Rc::new(std::cell::RefCell::new(ShaderState {
            mouse: (0.5, 0.3),
            start: js_sys::Date::now(),
            last_frame: 0.0,
        }));

        // Resize to match the canvas's own CSS-driven client box (the canvas
        // has `w-full h-full absolute inset-0` so the browser sizes it for
        // us). Reading `clientWidth/Height` after layout is the reliable way
        // to pick up the actual rendered box; `parent.getBoundingClientRect`
        // can return 0 during the first hydration pass.
        let resize = {
            let canvas = canvas.clone();
            let gl = gl.clone();
            move || {
                let dpr = web_sys::window()
                    .map(|w| w.device_pixel_ratio().min(2.0))
                    .unwrap_or(1.0)
                    * RENDER_SCALE;
                let cw = canvas.client_width().max(1) as f64;
                let ch = canvas.client_height().max(1) as f64;
                let w = (cw * dpr).max(1.0) as u32;
                let h = (ch * dpr).max(1.0) as u32;
                if canvas.width() != w {
                    canvas.set_width(w);
                }
                if canvas.height() != h {
                    canvas.set_height(h);
                }
                gl.viewport(0, 0, w as i32, h as i32);
            }
        };
        resize();
        // Run again on the next frame in case layout wasn't ready at mount.
        if let Some(window) = web_sys::window() {
            let once = {
                let resize = resize.clone();
                Closure::<dyn FnMut()>::once(move || resize())
            };
            let _ =
                window.request_animation_frame(once.as_ref().unchecked_ref::<js_sys::Function>());
            once.forget();
        }

        // Mouse tracking.
        let mouse_closure = {
            let state = state.clone();
            let canvas = canvas.clone();
            Closure::<dyn Fn(web_sys::PointerEvent)>::new(move |e: web_sys::PointerEvent| {
                let rect = canvas.get_bounding_client_rect();
                let x = ((e.client_x() as f64 - rect.left()) / rect.width()).clamp(0.0, 1.0);
                let y = ((e.client_y() as f64 - rect.top()) / rect.height()).clamp(0.0, 1.0);
                state.borrow_mut().mouse = (x as f32, y as f32);
            })
        };
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "pointermove",
                mouse_closure.as_ref().unchecked_ref(),
            );
        }
        mouse_closure.forget();

        // Resize listener.
        let resize_closure = {
            let resize = resize.clone();
            Closure::<dyn Fn()>::new(move || resize())
        };
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback(
                "resize",
                resize_closure.as_ref().unchecked_ref(),
            );
        }
        resize_closure.forget();

        // Render loop.
        let render_state = state.clone();
        let render_canvas = canvas.clone();
        let render_gl = gl.clone();
        let f: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut()>>>> =
            std::rc::Rc::new(std::cell::RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let Some(window) = web_sys::window() else {
                return;
            };

            let now = js_sys::Date::now();
            if now - render_state.borrow().last_frame < 1000.0 / 30.0 {
                if let Some(cb) = f.borrow().as_ref() {
                    let _ = window
                        .request_animation_frame(cb.as_ref().unchecked_ref::<js_sys::Function>());
                }
                return;
            }
            render_state.borrow_mut().last_frame = now;

            let s = render_state.borrow();
            let t = ((now - s.start) / 1000.0) as f32;
            let dark = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.document_element())
                .map(|el| el.class_name().contains("dark"))
                .unwrap_or(false);
            let dark_f = if dark { 1.0f32 } else { 0.0f32 };

            render_gl.uniform2f(
                u_resolution.as_ref(),
                render_canvas.width() as f32,
                render_canvas.height() as f32,
            );
            render_gl.uniform1f(u_time.as_ref(), t);
            render_gl.uniform2f(u_mouse.as_ref(), s.mouse.0, s.mouse.1);
            render_gl.uniform1f(u_dark.as_ref(), dark_f);

            render_gl.draw_arrays(GL::TRIANGLES, 0, 3);

            if let Some(cb) = f.borrow().as_ref() {
                let _ =
                    window.request_animation_frame(cb.as_ref().unchecked_ref::<js_sys::Function>());
            }
        }));

        if let Some(window) = web_sys::window()
            && let Some(cb) = g.borrow().as_ref()
        {
            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref::<js_sys::Function>());
        }
    });

    view! {
        <canvas
            node_ref=node
            class="fixed inset-0 w-screen h-screen -z-10 pointer-events-none"
            aria-hidden="true"
        />
    }
}

struct ShaderState {
    mouse: (f32, f32),
    start: f64,
    last_frame: f64,
}
