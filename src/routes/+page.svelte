<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let name = ''
    let greeting = ''
    let typedGreeting = ''
    let index = 0

    async function greet() {
        typedGreeting = ''
        index = 0
        if (name != '') {
            greeting = await invoke('greet', { name })
            typeGreeting()
        }
        else {
            greeting = 'Please enter something in the text input above.'
            typeGreeting()
        }
            
    }
    function typeGreeting() {
        if (typedGreeting.length != greeting.length) {
      typedGreeting += greeting.charAt(index);
      index++;
      setTimeout(typeGreeting, 20)
    }
    }
</script>

<div>  
    <input id="greet" placeholder="Enter a word..." bind:value="{name}"/>
    <button on:click="{greet}">Reverse</button>
    <div class="textbox"><p>{typedGreeting}</p></div>
</div>

<style>
    div {
        margin: auto;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        max-width: 100vw;
        max-height: 100vh;
        font-family: Arial, sans-serif;
        background: #f3f3f3;
        padding: 2rem;
    }

    input {
        font-size: 1rem;
        padding: 0.5rem;
        margin-bottom: 1rem;
        width: 100%;
        max-width: 300px;
        border: 1px solid #ccc;
        border-radius: 4px;
        outline: none;
        box-sizing: border-box;
    }

    input:focus {
        border-color: #007BFF;
    }

    button {
        font-size: 1rem;
        color: white;
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        background: #007BFF;
        transition: 0.3s ease-in-out;
        outline: none;
        width: 100%;
        max-width: 300px;
        box-sizing: border-box;
    }

    button:hover {
        background: #0056b3;
    }

    p {
        margin-top: 1rem;
        color: #333;
        font-size: 1.2rem;
    }
    .textbox {
        background-color: rgba(112, 128, 144, 0.201);
        height: 2em;
        min-width: 300px;
        margin: 25px auto;
        border-radius: 4px;
    }
</style>
