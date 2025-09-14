document.addEventListener('DOMContentLoaded', async () => {
    const form = document.getElementById('email-form');
    const authorSelect = document.getElementById('field');
    const publisherSelect = document.getElementById('field-2');
    const yearInput = document.querySelector('.text-field-5');
    const priceInput = document.querySelector('.text-field-3');
    const quantityInput = document.querySelector('.text-field-6');

    // Add input validation for numeric fields
    yearInput.addEventListener('input', (e) => {
        e.target.value = e.target.value.replace(/[^0-9]/g, '');
    });

    priceInput.addEventListener('input', (e) => {
        e.target.value = e.target.value.replace(/[^0-9.]/g, '');
    });

    quantityInput.addEventListener('input', (e) => {
        e.target.value = e.target.value.replace(/[^0-9]/g, '');
    });

    // Add empty option as first option
    authorSelect.innerHTML = '<option value="">Select Author</option>';
    publisherSelect.innerHTML = '<option value="">Select Publisher</option>';

    try {
        const [authorsResponse, publishersResponse] = await Promise.all([
            fetch('/api/authors/all'),
            fetch('/api/publishers/all')
        ]);

        const authors = await authorsResponse.json();
        const publishers = await publishersResponse.json();

        authors.forEach(author => {
            const option = document.createElement('option');
            option.value = author.id;
            option.textContent = author.name;
            authorSelect.appendChild(option);
        });

        publishers.forEach(publisher => {
            const option = document.createElement('option');
            option.value = publisher.id;
            option.textContent = publisher.name;
            publisherSelect.appendChild(option);
        });
    } catch (error) {
        console.error('Error loading data:', error);
        alert('Failed to load authors and publishers. Please refresh the page.');
    }

    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const title = document.getElementById('name').value.trim();
        const authorId = authorSelect.value;
        const publisherId = publisherSelect.value;
        const isbn = document.getElementById('name-2').value.trim();
        const year = yearInput.value.trim();
        const price = priceInput.value.trim();
        const quantity = quantityInput.value.trim();
        
        if (!title || !authorId || !publisherId || !isbn || !year || !price || !quantity) {
            alert("Please fill all fields");
            return;
        }

        const bookData = {
            title,
            author_id: parseInt(authorId),
            publisher_id: parseInt(publisherId),
            isbn,
            year: parseInt(year),
            price: parseFloat(price),
            quantity: parseInt(quantity)
        };

        console.log('Sending book data:', bookData);

        try {
            const response = await fetch('/api/books/create', {  // تغییر مسیر از /api/books/add به /api/books/new
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(bookData)
            });

            console.log('Response status:', response.status);
            const responseText = await response.text();
            console.log('Response text:', responseText);

            if (!response.ok) {
                throw new Error(responseText || 'Failed to add book');
            }

            alert('Book added successfully!');
            window.location.href = '/book/show-all';
        } catch (error) {
            console.error('Error:', error);
            alert(error.message);
        }
    });
});