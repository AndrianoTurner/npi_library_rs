<script>
	import { axiosClient } from '../../hooks.client';
	import Book from '../../components/Book.svelte';
	import { books } from '../../stores/book_store';
</script>

<ul>
	{#each $books as book}
		<li>
			<Book {book} />
		</li>
	{/each}
</ul>
