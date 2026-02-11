impl A2uiSurface {
    /// Render a ShaderStage component: selects the active shader effect
    /// based on data-bound effect name and sets audio amplitude + params for modulation.
    pub(crate) fn render_shader_stage(
        &mut self,
        cx: &mut Cx2d,
        stage: &ShaderStageComponent,
        data_model: &DataModel,
        _component_id: &str,
    ) {
        let effect_name = resolve_string_value_scoped(
            &stage.effect,
            data_model,
            self.current_scope.as_deref(),
        );
        let walk = Walk::new(Size::Fixed(stage.width), Size::Fixed(stage.height));
        let amp = self.audio_amplitude;

        // Resolve optional params with defaults
        let speed = stage.speed.as_ref().map_or(1.0, |v| {
            resolve_number_value_scoped(v, data_model, self.current_scope.as_deref()) as f32
        });
        let zoom = stage.zoom.as_ref().map_or(1.0, |v| {
            resolve_number_value_scoped(v, data_model, self.current_scope.as_deref()) as f32
        });
        let glow = stage.glow.as_ref().map_or(1.0, |v| {
            resolve_number_value_scoped(v, data_model, self.current_scope.as_deref()) as f32
        });
        let color_shift = stage.color_shift.as_ref().map_or(0.0, |v| {
            resolve_number_value_scoped(v, data_model, self.current_scope.as_deref()) as f32
        });

        macro_rules! draw_effect {
            ($draw:expr) => {{
                $draw.amplitude = amp;
                $draw.speed = speed;
                $draw.zoom = zoom;
                $draw.glow = glow;
                $draw.color_shift = color_shift;
                $draw.draw_walk(cx, walk);
            }};
        }

        match effect_name.as_str() {
            "aurora" => draw_effect!(self.draw_aurora),
            "reef" => draw_effect!(self.draw_reef),
            "fractalRainbow" => draw_effect!(self.draw_fractal_rainbow),
            "glowingLattice" => draw_effect!(self.draw_glowing_lattice),
            "jellyfish" => draw_effect!(self.draw_jellyfish),
            "turbulenceFire" => draw_effect!(self.draw_turbulence_fire),
            _ => draw_effect!(self.draw_aurora),
        }

        // Request continuous animation frame for shader time progression
        self.next_frame = cx.new_next_frame();
    }
}
