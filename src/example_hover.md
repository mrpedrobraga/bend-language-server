# Scene

A scene is like a little screenplay, with dialogs and commands for doing things in your app. You can connect triggers on your game's world to a scene and await on its execution.

```sol
scene "Example Scene"
  [Echo]
  - Hello there! Nice to see you!
  item.give(to: player, what: apple, amt: 1)
end
```

[Learn More](http://mrpedrobraga.com/nano)
