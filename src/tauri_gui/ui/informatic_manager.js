const { dialog } = window.__TAURI__.dialog
const { invoke } = window.__TAURI__.tauri

const state =
{
    informatic_handle: document.getElementById('informatic'),
    editable: false,
    text: "",
}

function draw_text(state) {
    let informatic_window = document.getElementById('informatic_window');
    if (state.editable) {
        state.informatic_handle.innerHTML = state.text;
        state.informatic_handle.contentEditable = true;
        informatic_window.classList.add('edit_mode')
    }
    else {
        const converter = new showdown.Converter();
        state.informatic_handle.innerHTML = converter.makeHtml(state.text);
        state.informatic_handle.contentEditable = false;
        informatic_window.classList.remove('edit_mode')
    }
}

function set_content(content) {
    state.text = content;
    draw_text(state)
}

function toggle_editable() {
    if (state.editable) {
        state.text = state.informatic_handle.innerText;
        save_current_text();
    }
    state.editable = !state.editable
    draw_text(state)
}

async function request_change_map() {
    return true
    // if (state.editable) {
    //     let confirm = await dialog.ask("This map has unsaved changes. Do you want to proceed?")
    //     if (confirm) {
    //         toggle_editable()
    //     }
    // }
    // else return true
}

async function save_current_text() {
    await invoke('update_map_content', { content: state.text }).catch((error) => {
        console.error(`Error saving content: ${error}`)
    });
}

// window.draw_text = draw_text
window.set_content = set_content
window.request_change_map = request_change_map


document.addEventListener('DOMContentLoaded', async () => {
    const edit_button = document.getElementById('edit_content_button');
    edit_button.addEventListener('click', () => { toggle_editable() });

    const resizer = document.getElementById('resizer');
    const informatic_window = document.getElementById('informatic_window');

    let originalX;
    let originalY;
    let originalMouseX;
    let originalMouseY;
    let window_width;
    let window_height;


    resizer.addEventListener('mousedown', (e) => {
        e.preventDefault();
        window_width = window.innerWidth / 100;
        window_height = window.innerHeight / 100;

        originalX = informatic_window.getBoundingClientRect().left / window_width;
        originalY = informatic_window.getBoundingClientRect().top / window_height;
        originalMouseX = e.pageX;
        originalMouseY = e.pageY;
        window.addEventListener('mousemove', resize);
        window.addEventListener('mouseup', stopResize);
    });

    function resize(e) {
        let newX = originalX + (e.pageX - originalMouseX) / window_width;
        let newY = originalY + (e.pageY - originalMouseY) / window_height;

        newY = newY > 1 ? newY : 0
        newX = newX < 92 ? newX : 92

        informatic_window.style.left = newX + '%';
        informatic_window.style.top = newY + '%';

        informatic_window.style.width = 100 - newX + '%';
        informatic_window.style.height = 100 - newY + '%';
    }

    function stopResize() {
        window.removeEventListener('mousemove', resize);
        window.removeEventListener('mouseup', stopResize);
    }
});



