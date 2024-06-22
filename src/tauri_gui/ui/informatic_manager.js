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
});


// let dragEl
// let dragHandleEl
// const lastPosition = {};

// setupDraggable();

// function setupDraggable() {
//     dragHandleEl = document.querySelector('informatic');
//     dragHandleEl.addEventListener('mousedown', dragStart);
//     dragHandleEl.addEventListener('mouseup', dragEnd);
//     dragHandleEl.addEventListener('mouseout', dragEnd);
// }

// // function setupResizable() {
// //     const resizeEl = document.querySelector('[data-resizable]');
// //     resizeEl.style.setProperty('resize', 'both');
// //     resizeEl.style.setProperty('overflow', 'hidden');
// // }

// function dragStart(event) {
//     dragEl = getDraggableAncestor(event.target);
//     dragEl.style.setProperty('position', 'absolute');
//     lastPosition.left = event.target.clientX;
//     lastPosition.top = event.target.clientY;
//     dragHandleEl.classList.add('dragging');
//     dragHandleEl.addEventListener('mousemove', dragMove);
// }

// function dragMove(event) {
//     const dragElRect = dragEl.getBoundingClientRect();
//     const newLeft = dragElRect.left + event.clientX - lastPosition.left;
//     const newTop = dragElRect.top + event.clientY - lastPosition.top;
//     dragEl.style.setProperty('left', `${newLeft}px`);
//     dragEl.style.setProperty('top', `${newTop}px`);
//     lastPosition.left = event.clientX;
//     lastPosition.top = event.clientY;
//     window.getSelection().removeAllRanges();
// }

// function getDraggableAncestor(element) {
//     if (element.getAttribute('data-draggable')) return element;
//     return getDraggableAncestor(element.parentElement);
// }

// function dragEnd() {
//     dragHandleEl.classList.remove('dragging');
//     dragHandleEl.removeEventListener('mousemove', dragMove);
//     dragEl = null;
// }
