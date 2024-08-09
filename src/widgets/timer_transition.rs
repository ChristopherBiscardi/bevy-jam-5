use bevy::prelude::*;
use interpolation::{Ease, EaseFunction};
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
    /// The easing function that dictates the
    /// interpolation factor.
    pub easing: TransitionEasing,
    /// Indicates the direction of the animation
    pub reversing: bool,
    /// The start time of the animation.
    pub start: Timer,
    /// The time until the animation completed.
    pub timeouts: Vec<Timer>,
    /// current timer index
    pub current_index: usize,
    /// Does the animation loop?
    ///
    /// TODO: Change from boolean to enum that
    /// allows disabling the reversing animation.
    pub looping: bool,
    /// The styles of the widget.
    pub styles: Vec<WoodpeckerStyle>,
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
            timeouts: vec![Timer::new(
                Duration::from_millis(2000),
                TimerMode::Once,
            )],
            current_index: 0,
            looping: Default::default(),
            styles: Default::default(),
        }
    }
}
impl TransitionTimer {
    pub(crate) fn update(
        &mut self,
        time: Time,
    ) -> Option<WoodpeckerStyle> {
        if self
            .timeouts
            .iter()
            .all(|timer| timer.finished())
        {
            return None;
        }
        // if we haven't started, tick start timer
        if !(self.start.finished()
            || self
                .start
                .tick(time.delta())
                .just_finished())
        {
            return Some(self.styles[0]);
        };

        let Some(next_timer) =
            self.timeouts.get_mut(self.current_index)
        else {
            // TODO: lengths must match in constructor?
            return Some(self.styles[self.current_index]);
        };
        // if we haven't finished the animation, play
        // animation
        if next_timer.finished() {
            return Some(self.styles[self.current_index]);
        };

        if next_timer.tick(time.delta()).just_finished() {
            self.current_index += 1;
            return Some(self.styles[self.current_index]);
        }

        // if self.playing {
        let mut x = if let Some(easing) =
            self.easing.try_into_easing_function()
        {
            Ease::calc(next_timer.fraction(), easing)
        } else {
            next_timer.fraction()
        };
        if self.reversing {
            x = 1.0 - x;
        }

        Some(
            self.styles[self.current_index].lerp(
                &self.styles[self.current_index + 1],
                x,
            ),
        )

        // } else if self.looping && self.playing
        // {     // Restart animation
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
        let Some(new_styles) = transition.update(*time)
        else {
            // if the transition isn't playing, don't update
            // style
            continue;
        };
        *styles = new_styles;
    }
}
