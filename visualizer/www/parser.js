const rust = import('../pkg');

rust
    .then(m => {
        m.greet(3.5, 12);
    })
    .catch(console.error);