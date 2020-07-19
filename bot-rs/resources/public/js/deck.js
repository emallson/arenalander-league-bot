function arena_export() {
    let rows = Array.from(document.querySelectorAll('.deck-sections li'));
    let result = "Deck\r\n" + rows.map(row => {
        let count = row.querySelector('.card-count').innerText;
        let name = row.querySelector('.mtgcard .card-link').innerText.replace('/', '//');
        return `${count} ${name}`;
    }).join('\r\n');

    navigator.clipboard.writeText(result)
             .then(() => document.querySelector('#copy-link').classList.add('copy-success'))
             .catch(err => alert("Unable to copy to clipboard"));
}
