<script>
	import BookEdit from './BookEdit.svelte';

	export let book;
</script>

<a href={book.link()}>{book.title} - {book.discipline}</a>
