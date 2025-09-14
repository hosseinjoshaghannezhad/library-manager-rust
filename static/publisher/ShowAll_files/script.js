// متغیرهای سراسری برای نگهداری اطلاعات
let publishers = [];
let books = [];

// دریافت اطلاعات ناشران و کتاب‌ها
async function loadData() {
    try {
        // دریافت ناشران
        const publishersResponse = await fetch('/api/publishers/all');
        if (publishersResponse.ok) {
            publishers = await publishersResponse.json();
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

// شمارش تعداد کتاب‌های هر ناشر
function getBookCount(publisherId) {
    return books.filter(book => book.publisher_id === publisherId && !book.is_deleted).length;
}

// دریافت داده‌ها از API و پر کردن جدول
async function populateTable() {
    try {
        const tbody = document.querySelector('.giving-back-table tbody');
        tbody.innerHTML = ''; // پاک کردن محتوای قبلی

        // فیلتر کردن فقط ناشران حذف نشده
        const activePublishers = publishers.filter(publisher => !publisher.is_deleted);

        activePublishers.forEach(publisher => {
            const tr = document.createElement('tr');
            tr.style.borderBottom = '1px solid #e5e5e5';
            tr.innerHTML = `
                <td style="padding: 10px;"><input type="checkbox" class="publisher-checkbox" data-id="${publisher.id}"></td>
                <td style="padding: 10px;">${publisher.name}</td>
                <td style="padding: 10px;">${getBookCount(publisher.id)}</td>
                <td style="padding: 10px;">
                    <button class="button-small" 
                    style="background-color: #4CAF50; border-radius: 15px; border: none; color: white; padding: 5px 10px; cursor: pointer;">Show</button>
                </td>
                <td style="padding: 10px;">
                    <button onclick="deletePublisher(${publisher.id})" class="button-small delete"
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
    const checkboxes = document.querySelectorAll('.publisher-checkbox');
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

// حذف یک ناشر
async function deletePublisher(id) {
    if (!confirm('Are you sure you want to delete this publisher?')) {
        return;
    }

    try {
        const response = await fetch(`/api/publishers/${id}`, {
            method: 'DELETE'
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        // بعد از حذف، جدول رو دوباره پر می‌کنیم
        populateTable();
    } catch (error) {
        console.error('Error deleting publisher:', error);
    }
}

// جستجو در ناشران
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
                const checkbox = row.querySelector('.publisher-checkbox');
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
    
    // اضافه کردن event listener برای لینک LiMa
    document.querySelector('a.button').addEventListener('click', (e) => {
        e.preventDefault();
        window.location.href = '/';
    });
}); 