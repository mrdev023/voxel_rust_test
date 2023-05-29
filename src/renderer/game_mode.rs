use std::sync::Arc;
use wgpu::Queue;
use winit::event::Event;
use async_trait::async_trait;
use crate::renderer::graphics_renderer::GraphicsRenderer;

#[async_trait]
pub trait GameMode {
    async fn new(renderer: &mut Arc<GraphicsRenderer>) -> Self;

    fn input(&mut self, event: &Event<()>) -> bool;

    fn update(&mut self, dt: instant::Duration, renderer: &mut Arc<GraphicsRenderer>);

    fn render(&mut self, renderer: &mut Arc<GraphicsRenderer>);

    async fn load(&mut self, renderer: &mut Arc<GraphicsRenderer>);

    async fn unload(&mut self, renderer: &mut Arc<GraphicsRenderer>);
}