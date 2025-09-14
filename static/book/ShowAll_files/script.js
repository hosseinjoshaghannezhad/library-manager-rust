// متغیرهای سراسری برای نگهداری اطلاعات
let authors = [];
let publishers = [];

// دریافت اطلاعات نویسنده‌ها و ناشران
async function loadData() {
    try {
        // دریافت نویسنده‌ها
        const authorsResponse = await fetch('/api/authors/all');
        if (authorsResponse.ok) {
            authors = await authorsResponse.json();
        }

        // دریافت ناشران
        const publishersResponse = await fetch('/api/publishers/all');
        if (publishersResponse.ok) {
            publishers = await publishersResponse.json();
        }

        // حالا جدول رو پر میکنیم
        await populateTable();
    } catch (error) {
        console.error('Error loading data:', error);
    }
}

// پیدا کردن نام نویسنده با آیدی
function getAuthorName(authorId) {
    const author = authors.find(a => a.id === authorId);
    return author ? author.name : '-';
}

// پیدا کردن نام ناشر با آیدی
function getPublisherName(publisherId) {
    const publisher = publishers.find(p => p.id === publisherId);
    return publisher ? publisher.name : '-';
}

// دریافت داده‌ها از API و پر کردن جدول
async function populateTable() {
    try {
        const response = await fetch('/api/books/all');
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const books = await response.json();
        
        const tbody = document.querySelector('.giving-back-table tbody');
        tbody.innerHTML = ''; // پاک کردن محتوای قبلی

        // فیلتر کردن فقط کتاب‌های حذف نشده
        const activeBooks = books.filter(book => !book.is_deleted);

        activeBooks.forEach(book => {
            const tr = document.createElement('tr');
            tr.style.borderBottom = '1px solid #e5e5e5';
            tr.innerHTML = `
                <td style="padding: 10px;"><input type="checkbox" class="book-checkbox" data-id="${book.id}"></td>
                <td style="padding: 10px;">${book.title}</td>
                <td style="padding: 10px;">${getAuthorName(book.author_id)}</td>
                <td style="padding: 10px;">${getPublisherName(book.publisher_id)}</td>
                <td style="padding: 10px;">${book.isbn || '-'}</td>
                <td style="padding: 10px;">
                    <button class="button-small" 
                    style="background-color: #4CAF50; border-radius: 15px; border: none; color: white; padding: 5px 10px; cursor: pointer;">Show</button>
                </td>
                <td style="padding: 10px;">
                    <button onclick="deleteBook(${book.id})" class="button-small delete"
                    style="background-color: #f44336; border-radius: 15px; border: none; color: white; padding: 5px 10px; cursor: pointer;">Delete</button>
                </td>
            `;
            tbody.appendChild(tr);
        });

        // اضافه کردن event listener برای چک‌باکس‌ها
        setupCheckboxListeners();
    } catch (error) {
        console.error('Error fetching books:', error);
    }
}

// تنظیم event listener برای چک‌باکس‌ها
function setupCheckboxListeners() {
    const checkboxes = document.querySelectorAll('.book-checkbox');
    checkboxes.forEach(checkbox => {
        checkbox.addEventListener('change', () => {
            const row = checkbox.closest('tr');
            if (checkbox.checked) {
                // اگر چک شد، به بالای جدول منتقل کن
                const tbody = document.querySelector('.giving-back-table tbody');
                tbody.insertBefore(row, tbody.firstChild);
            }
        });
    });
}

// حذف یک کتاب
async function deleteBook(id) {
    if (!confirm('Are you sure you want to delete this book?')) {
        return;
    }

    try {
        const response = await fetch(`/api/books/${id}`, {
            method: 'DELETE'
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        // بعد از حذف، جدول رو دوباره پر می‌کنیم
        populateTable();
    } catch (error) {
        console.error('Error deleting book:', error);
    }
}

// جستجو در کتاب‌ها
function setupSearch() {
    const searchInput = document.getElementById('field');
    let timeoutId;

    searchInput.addEventListener('input', (e) => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            const searchTerm = e.target.value.toLowerCase();
            const rows = document.querySelectorAll('.giving-back-table tbody tr');
            
            rows.forEach(row => {
                const title = row.children[1].textContent.toLowerCase();
                const author = row.children[2].textContent.toLowerCase();
                const publisher = row.children[3].textContent.toLowerCase();
                const isbn = row.children[4].textContent.toLowerCase();
                const checkbox = row.querySelector('.book-checkbox');
                const isChecked = checkbox && checkbox.checked;
                
                // اگر چک‌باکس تیک خورده باشه، همیشه نمایش بده
                // در غیر این صورت، فقط اگر با عبارت جستجو مطابقت داشت نمایش بده
                if (isChecked || 
                    title.includes(searchTerm) || 
                    author.includes(searchTerm) || 
                    publisher.includes(searchTerm) || 
                    isbn.includes(searchTerm)) {
                    row.style.display = '';
                } else {
                    row.style.display = 'none';
                }
            });
        }, 300);
    });
}

// راه‌اندازی اولیه
document.addEventListener('DOMContentLoaded', () => {
    // لود کردن داده‌های اولیه
    loadData();
    
    // راه‌اندازی جستجو
    setupSearch();
    
    // اضافه کردن event listener برای لینک LiMa
    document.querySelector('a.button').addEventListener('click', (e) => {
        e.preventDefault();
        window.location.href = '/';
    });
}); 