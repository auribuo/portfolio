document.body.onload = () => {
    new IntersectionObserver((entries, _o) => {
        entries.forEach(entry =>
            entry.scrollIntoView()
        )
    }, {
        root: null,
        threshold: .5
    }).observe(document.getElementById('input'))
}