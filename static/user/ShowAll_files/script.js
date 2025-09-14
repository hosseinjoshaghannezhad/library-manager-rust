// متغیرهای سراسری برای نگهداری اطلاعات
let users = [];

// دریافت اطلاعات کاربران
async function loadData() {
    try {
        // دریافت کاربران
        const usersResponse = await fetch('/api/users/all');
        if (usersResponse.ok) {
            users = await usersResponse.json();
        }

        // حالا جدول رو پر میکنیم
        await populateTable();
    } catch (error) {
        console.error('Error loading data:', error);
    }
}

// دریافت داده‌ها از API و پر کردن جدول
async function populateTable() {
    try {
        const tbody = document.querySelector('.giving-back-table tbody');
        tbody.innerHTML = ''; // پاک کردن محتوای قبلی

        // فیلتر کردن فقط کاربران حذف نشده
        const activeUsers = users.filter(user => !user.is_deleted);

        activeUsers.forEach(user => {
            const tr = document.createElement('tr');
            tr.style.borderBottom = '1px solid #e5e5e5';
            tr.innerHTML = `
                <td style="padding: 10px;"><input type="checkbox" class="user-checkbox" data-id="${user.id}"></td>
                <td style="padding: 10px;">${user.name}</td>
                <td style="padding: 10px;">${user.membership_id || '-'}</td>
                <td style="padding: 10px;">
                    <button class="button-small" 
                    style="background-color: #4CAF50; border-radius: 15px; border: none; color: white; padding: 5px 10px; cursor: pointer;">Show</button>
                </td>
                <td style="padding: 10px;">
                    <button onclick="deleteUser(${user.id})" class="button-small delete"
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
    const checkboxes = document.querySelectorAll('.user-checkbox');
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

// حذف یک کاربر
async function deleteUser(id) {
    if (!confirm('Are you sure you want to delete this user?')) {
        return;
    }

    try {
        const response = await fetch(`/api/users/${id}`, {
            method: 'DELETE'
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }

        // بعد از حذف، جدول رو دوباره پر می‌کنیم
        populateTable();
    } catch (error) {
        console.error('Error deleting user:', error);
    }
}

// جستجو در کاربران
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
                const membershipId = row.children[2].textContent.toLowerCase();
                const checkbox = row.querySelector('.user-checkbox');
                const isChecked = checkbox && checkbox.checked;
                
                // اگر چک‌باکس تیک خورده باشه، همیشه نمایش بده
                // در غیر این صورت، فقط اگر با عبارت جستجو مطابقت داشت نمایش بده
                if (isChecked || name.includes(searchTerm) || membershipId.includes(searchTerm)) {
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