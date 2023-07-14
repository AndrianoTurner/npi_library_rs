<script>
	import { axiosClient } from '../../hooks.client';

	let book_array = [
		{
			book_id: 1,
			title: 'SampleTitle',
			discipline: 'Электроника',
			url: 'book/1'
		},

		{
			book_id: 2,
			title: 'SampleTitl2',
			discipline: 'Электроника',
			url: 'http://book2.html'
		}
	];
	function get_all_books() {
		axiosClient.get('office/books').then(function (response) {
			console.log(response);
		});
	}
</script>

<ul>
	{#each book_array as book}
		<li>
			<a href={book.url}>{book.discipline} - {book.title}</a>
		</li>
	{/each}
</ul>

<button on:click={get_all_books}>Get Books</button>
