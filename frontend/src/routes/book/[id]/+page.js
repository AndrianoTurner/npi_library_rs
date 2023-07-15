/** @type {import('./$types').PageLoad} */
import {get,writable} from 'svelte/store'
import Book from '../../../book'
import { books } from '../../../stores/book_store';
export function load({ params }) {
    let book_array = get(books)

   let book = book_array.find((element,index,array) => {
        return element.id == params.id
    } )

    return {book};
}