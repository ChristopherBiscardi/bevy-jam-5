use bevy::prelude::*;
use interpolation::Ease;
use interpolation::EaseFunction;
use std::time::{Duration, Instant};

use woodpecker_ui::prelude::*;

#[derive(Default, Copy, Clone, PartialEq)]
pub enum TransitionEasing {
    #[default]
    Linear,
    QuadraticIn,
    QuadraticOut,
    QuadraticInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuarticIn,
    QuarticOut,
    QuarticInOut,
    QuinticIn,
    QuinticOut,
    QuinticInOut,
    SineIn,
    SineOut,
    SineInOut,
    CircularIn,
    CircularOut,
    CircularInOut,
    ExponentialIn,
    ExponentialOut,
    ExponentialInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

impl TransitionEasing {
    fn try_into_easing_function(
        &self,
    ) -> Option<EaseFunction> {
        match self {
            TransitionEasing::QuadraticIn => {
                Some(EaseFunction::QuadraticIn)
            }
            TransitionEasing::QuadraticOut => {
                Some(EaseFunction::QuadraticOut)
            }
            TransitionEasing::QuadraticInOut => {
                Some(EaseFunction::QuadraticInOut)
            }
            TransitionEasing::CubicIn => {
                Some(EaseFunction::CubicIn)
            }
            TransitionEasing::CubicOut => {
                Some(EaseFunction::CubicOut)
            }
            TransitionEasing::CubicInOut => {
                Some(EaseFunction::CubicInOut)
            }
            TransitionEasing::QuarticIn => {
                Some(EaseFunction::QuarticIn)
            }
            TransitionEasing::QuarticOut => {
                Some(EaseFunction::QuarticOut)
            }
            TransitionEasing::QuarticInOut => {
                Some(EaseFunction::QuarticInOut)
            }
            TransitionEasing::QuinticIn => {
                Some(EaseFunction::QuinticIn)
            }
            TransitionEasing::QuinticOut => {
                Some(EaseFunction::QuinticOut)
            }
            TransitionEasing::QuinticInOut => {
                Some(EaseFunction::QuinticInOut)
            }
            TransitionEasing::SineIn => {
                Some(EaseFunction::SineIn)
            }
            TransitionEasing::SineOut => {
                Some(EaseFunction::SineOut)
            }
            TransitionEasing::SineInOut => {
                Some(EaseFunction::SineInOut)
            }
            TransitionEasing::CircularIn => {
                Some(EaseFunction::CircularIn)
            }
            TransitionEasing::CircularOut => {
                Some(EaseFunction::CircularOut)
            }
            TransitionEasing::CircularInOut => {
                Some(EaseFunction::CircularInOut)
            }
            TransitionEasing::ExponentialIn => {
                Some(EaseFunction::ExponentialIn)
            }
            TransitionEasing::ExponentialOut => {
                Some(EaseFunction::ExponentialOut)
            }
            TransitionEasing::ExponentialInOut => {
                Some(EaseFunction::ExponentialInOut)
            }
            TransitionEasing::ElasticIn => {
                Some(EaseFunction::ElasticIn)
            }
            TransitionEasing::ElasticOut => {
                Some(EaseFunction::ElasticOut)
            }
            TransitionEasing::ElasticInOut => {
                Some(EaseFunction::ElasticInOut)
            }
            TransitionEasing::BackIn => {
                Some(EaseFunction::BackIn)
            }
            TransitionEasing::BackOut => {
                Some(EaseFunction::BackOut)
            }
            TransitionEasing::BackInOut => {
                Some(EaseFunction::BackInOut)
            }
            TransitionEasing::BounceIn => {
                Some(EaseFunction::BounceIn)
            }
            TransitionEasing::BounceOut => {
                Some(EaseFunction::BounceOut)
            }
            TransitionEasing::BounceInOut => {
                Some(EaseFunction::BounceInOut)
            }
            _ => None,
        }
    }
}

#[derive(Component, Widget, Clone, PartialEq)]
pub struct TransitionTimer {
    pub playing: bool,
    /// The easing function that dictates the interpolation factor.
    pub easing: TransitionEasing,
    /// Indicates the direction of the animation
    pub reversing: bool,
    /// The start time of the animation.
    pub start: Timer,
    /// The time until the animation completed.
    pub timeout: Timer,
    /// Does the animation loop?
    ///
    /// TODO: Change from boolean to enum that allows disabling the reversing animation.
    pub looping: bool,
    /// The starting styles of the widget.
    pub style_a: WoodpeckerStyle,
    /// The ending styles of the widget.
    pub style_b: WoodpeckerStyle,
}

impl Default for TransitionTimer {
    fn default() -> Self {
        Self {
            playing: true,
            easing: Default::default(),
            reversing: Default::default(),
            start: Timer::new(
                Duration::from_millis(0),
                TimerMode::Once,
            ),
            timeout: Timer::new(
                Duration::from_millis(2000),
                TimerMode::Once,
            ),
            looping: Default::default(),
            style_a: Default::default(),
            style_b: Default::default(),
        }
    }
}
impl TransitionTimer {
    pub(crate) fn update(
        &mut self,
        time: Time,
    ) -> WoodpeckerStyle {
        // if we haven't started, tick start timer
        if !(self.start.finished()
            || self
                .start
                .tick(time.delta())
                .just_finished())
        {
            dbg!("style a");
            return self.style_a;
        };

        // if we haven't finished the animation, play animation
        if self.timeout.finished()
            || self
                .timeout
                .tick(time.delta())
                .just_finished()
        {
            dbg!("style b");
            return self.style_b;
        };

        dbg!("playing");
        // if self.playing {
        let mut x = if let Some(easing) =
            self.easing.try_into_easing_function()
        {
            Ease::calc(self.timeout.fraction(), easing)
        } else {
            self.timeout.fraction()
        };
        if self.reversing {
            x = 1.0 - x;
        }
        self.style_a.lerp(&self.style_b, x)
        // } else if self.looping && self.playing {
        //     // Restart animation
        //     self.start = Instant::now();
        //     self.reversing = !self.reversing;
        //     if self.reversing {
        //         self.style_b
        //     } else {
        //         self.style_a
        //     }
        // } else {
        //     // End of animation
        //     self.playing = false;
        //     if self.reversing {
        //         self.style_a
        //     } else {
        //         self.style_b
        //     }
        // }
    }

    // /// Is the animation currently playing?
    // pub fn is_playing(&self) -> bool {
    //     self.playing
    // }

    // /// Starts the animation.
    // pub fn start(&mut self) {
    //     self.reversing = false;
    //     self.start = Instant::now();
    //     self.playing = true;
    // }

    // /// Starts the animation in reverse.
    // pub fn start_reverse(&mut self) {
    //     self.reversing = true;
    //     self.start = Instant::now();
    //     self.playing = true;
    // }
}

pub fn update_transitions(
    mut query: Query<(
        &mut TransitionTimer,
        &mut WoodpeckerStyle,
    )>,
    time: Res<Time>,
) {
    for (mut transition, mut styles) in query.iter_mut() {
        let new_styles = transition.update(*time);
        *styles = new_styles;
    }
}
