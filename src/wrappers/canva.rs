// // persona-like ui not exact to avoid copy right
// #[component]
// pub fn FutureSelector() -> impl IntoView {
//     let canvas_node: NodeRef<Canvas> = create_node_ref();

//     let selection = move |ev: MouseEvent| {
//         let ctx = canvas_node
//             .get()
//             .expect("Canvas to be placed")
//             .get_context("2d")
//             .ok()
//             .flatten()
//             .expect("Canvas to have 2d context")
//             .unchecked_into::<CanvasRenderingContext2d>();

        
//         // Journey
//         wheel(ctx);
//     };

//     view! {
//         <canvas
//             node_ref=canvas_node
//             on:mouseover=selection
//             id="home-canvas"
//             width="800" height="800"
//         >

//         </canvas>
//     }
// }

// pub fn wheel(ctx: CanvasRenderingContext2d) -> CanvasRenderingContext2d {
        
//     ctx.set_fill_style(&JsValue::from_str("blue"));
//     ctx.fill_rect(20.0, 20.0, 20.0, 20.0);

//     // ctx.set_font("bold 48px serif");
//     // ctx.fill_text("Zone!", 80.0, 80.0).ok();
//     // ctx.fill_text("Tool!", 80.0, 80.0).ok();
//     // ctx.fill_text("About!", 80.0, 80.0).ok();
//     // ctx.fill_text("Download!", 80.0, 80.0).ok();


//     ctx
// }





