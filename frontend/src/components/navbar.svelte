<script>
	import { auth_store } from '../stores/auth';
</script>

<nav>
	<a href="/">Домой</a>
	{#if $auth_store == null}
		<a href="/login">Вход</a>
		<a href="/register">Регистрация</a>
	{:else}
		<a href="/books">Книги</a>
		<a href="/logout">Выход</a>
	{/if}
</nav>
