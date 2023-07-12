<script>
	import { axiosClient } from "../../hooks.client";

    function get_all_books(){
        axiosClient.get("office/books").then(function (response){
            console.log(response)
        })
    }
</script>


<button on:click={get_all_books}>Get Books</button>