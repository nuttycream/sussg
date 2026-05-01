if (!window.matchMedia('(pointer: coarse)').matches) {
(() => {
    const cuh = 50;
    const buh = 500;

    document.addEventListener('keydown', (e) => {

        // pls dont steal input lulw
        const tag = document.activeElement.tagName;
        if (tag === 'INPUT' 
            || tag === 'TEXTAREA' 
            || document.activeElement.isContentEditable) {
            return
        };


        switch (e.key) {
            case 'j': window.scrollBy(0, cuh); break;
            case 'k': window.scrollBy(0, -cuh); break;
            case '}': window.scrollBy(0, buh); break;
            case '{': window.scrollBy(0, -buh); break;
            case 'g': if (!e.repeat) window.scrollTo(0, 0); break;
            case 'G': window.scrollTo(0, document.body.scrollHeight); break;
        }
    });

    const bindings = [
        ['k','up'],
        ['j','down'],
        ['{','jump up'],
        ['}','jump down'],
        ['g','top'],
        ['G','bottom'],
    ];

    const el = document.createElement('div');

    el.id = 'vim-help';
    el.innerHTML = bindings.map(([key, desc]) =>
        `<span class="vim-key">
            ${key}
        </span>
        <span class="vim-desc">
            ${desc}
        </span>`
    ).join('');

    document.body.appendChild(el);
})();

}
