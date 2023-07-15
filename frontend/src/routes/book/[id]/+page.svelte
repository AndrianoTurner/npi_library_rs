<script>
	import Book from '../../../components/Book.svelte';
	import BookEdit from '../../../components/BookEdit.svelte';
	/** @type {import('./$types').PageData} */
	export let data;
	console.log(data);
</script>

<BookEdit book={data.book} />
