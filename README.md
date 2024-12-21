The contents of `lib.rs` will cause a panic in clippy 1.83. It also affects
older versions of clippy. See the CI testing matrix for reproductions that
use recent clippy versions and different hosts.

The ICE disappears if we apply [the commit] in the `fix` branch.

[the commit]: https://github.com/mciantyre/clippy-1.83-ice/commit/2722bb92d67b2e6a8c4a61d4e8f78c267f1f1675

I first noticed this in a more complex [example], within the imxrt-hal project.

[example]: https://github.com/mciantyre/imxrt-hal/actions/runs/12445217329/job/34746314163
