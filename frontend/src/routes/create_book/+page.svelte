<script>
	export let book;

	function create_book() {
        let create_file_info = {
            
        }
    }
</script>

<input bind:value={book.title} placeholder="Заголовок" />
<input bind:value={book.discipline} placeholder="Дисциплина" />

<button on:click={create_book}>Сохранить</button>
