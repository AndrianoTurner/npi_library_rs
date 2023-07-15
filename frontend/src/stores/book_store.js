import {writable} from "svelte/store";
import Book from '../book'
export const books = writable([]);

function get_books(){
    
    let book_array = [
		new Book(
			1,
			'SampleTitle',
			'Электроника',
            1,
            "aboba.docx",
            `http://10.0.0.27:8080/api/office/download/8/aboba.docx`,
            `http://10.0.0.27:8080/api/office/track/8/aboba.docx`,
            `docx`,
            'W7FAFC9C21A8',
            `word`
        ),

	];
    books.set(book_array)
}

get_books()

