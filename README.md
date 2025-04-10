# Bevy, where was I?

A tiny Bevy library that saves the camera position when the program closes, and restores it when
you launch the application again. This is useful when debugging and you don't want to continuously
change your hardcoded camera coordinates.

Note that this actually works for any `Transform` component, and not only restores translation
information, but also scale and rotation.

This crate is compatible with [bevy_panorbit_camera](https://github.com/Plonq/bevy_panorbit_camera).

## Usage

Add the plugin:

```rust ignore
.add_plugins(WhereWasIPlugin::default())
```

Add the `WhereWasI` component to an entity with a `Transform`:

```rust ignore
commands.spawn((
    Transform::from_xyz(5.0, 2.0, 5.0),
    WhereWasI {
        name: "camera".into(),
    },
));
```

See the
[3D scene example](https://github.com/evroon/bevy-where-was-i/blob/master/examples/3d_scene.rs)
for more information.

## License

Bracket is licensed under [MIT](https://choosealicense.com/licenses/mit/), see [LICENSE](LICENSE).
