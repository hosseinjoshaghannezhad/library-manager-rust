// متغیرهای سراسری برای نگهداری اطلاعات
let authors = [];
let books = [];

// دریافت اطلاعات نویسنده‌ها و کتاب‌ها
async function loadData() {
    try {
        // دریافت نویسنده‌ها
        const authorsResponse = await fetch('/api/authors/all');
        if (authorsResponse.ok) {
            authors = await authorsResponse.json();
        }

        // دریافت کتاب‌ها برای شمارش
        const booksResponse = await fetch('/api/books/all');
        if (booksResponse.ok) {
            books = await booksResponse.json();
        }

        // حالا جدول رو پر میکنیم
        await populateTable();
    } catch (error) {
        console.error('Error loading data:', error);
    }
}

// شمارش تعداد کتاب‌های هر نویسنده
function getBookCount(authorId) {
    return books.filter(book => book.author_id === authorId && !book.is_deleted).length;
}

// دریافت داده‌ها از API و پر کردن جدول
async function populateTable() {
    try {
        const tbody = document.querySelector('.giving-back-table tbody');
        tbody.innerHTML = ''; // پاک کردن محتوای قبلی

        // فیلتر کردن فقط نویسنده‌های حذف نشده
        const activeAuthors = authors.filter(author => !author.is_deleted);

        activeAuthors.forEach(author => {
            const tr = document.createElement('tr');
            tr.style.borderBottom = '1px solid #e5e5e5';
            tr.innerHTML = `
                <td style="padding: 10px;"><input type="checkbox" class="author-checkbox" data-id="${author.id}"></td>
                <td style="padding: 10px;">${author.name}</td>
                <td style="padding: 10px;">${getBookCount(author.id)}</td>
                <td style="padding: 10px;">
                    <button class="button-small" 
                    style="background-color: #4CAF50; border-radius: 15px; border: none; color: white; padding: 5px 10px; cursor: pointer;">Show</button>
                </td>
                <td style="padding: 10px;">
                    <button onclick="deleteAuthor(${author.id})" class="button-small delete"
                    style="background-color: #f44336; border-radius: 15px; border: none; color: white; padding: 5px 10px; cursor: pointer;">Delete</button>
                </td>
            `;
            tbody.appendChild(tr);
        });

        // اضافه کردن event listener برای چک‌باکس‌ها
        setupCheckboxListeners();
    } catch (error) {
        console.error('Error populating table:', error);
    }
}

// تنظیم event listener برای چک‌باکس‌ها
function setupCheckboxListeners() {
    const checkboxes = document.querySelectorAll('.author-checkbox');
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

// حذف یک نویسنده
async function deleteAuthor(id) {
    if (!confirm('Are you sure you want to delete this author?')) {
        return;
    }

    try {
        const response = await fetch(`/api/authors/${id}`, {
            method: 'DELETE'
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        // بعد از حذف، جدول رو دوباره پر می‌کنیم
        populateTable();
    } catch (error) {
        console.error('Error deleting author:', error);
    }
}

// جستجو در نویسنده‌ها
function setupSearch() {
    const searchInput = document.getElementById('field');
    let timeoutId;

    searchInput.addEventListener('input', (e) => {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => {
            const searchTerm = e.target.value.toLowerCase();
            const rows = document.querySelectorAll('.giving-back-table tbody tr');
            
            rows.forEach(row => {
                const name = row.children[1].textContent.toLowerCase();
                const checkbox = row.querySelector('.author-checkbox');
                const isChecked = checkbox && checkbox.checked;
                
                // اگر چک‌باکس تیک خورده باشه، همیشه نمایش بده
                // در غیر این صورت، فقط اگر با عبارت جستجو مطابقت داشت نمایش بده
                if (isChecked || name.includes(searchTerm)) {
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
    
    // // اضافه کردن event listener برای لینک LiMa
    // document.querySelector('a.button').addEventListener('click', (e) => {
    //     e.preventDefault();
    //     window.location.href = '/';
    // });
}); 