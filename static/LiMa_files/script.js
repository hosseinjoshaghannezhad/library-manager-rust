// پر کردن اولیه سلکت‌باکس‌ها با داده‌های واقعی از سرور
async function populateUsers() {
    const select = document.getElementById("field-3");
    if (!select) {
        console.error("User select element (field-3) not found!");
        return;
    }
    try {
        const response = await fetch("/api/users/all");
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const users = await response.json();
        console.log("Users fetched:", users);
        select.innerHTML = '<option value="">Select one...</option>';
        users.filter(user => !user.is_deleted).forEach(user => {
            select.innerHTML += `<option value="${user.id}">${user.name || user.membership_id}</option>`;
        });
    } catch (error) {
        console.error("Error fetching users:", error);
    }
}

async function populateBooks() {
    const select = document.getElementById("field-2");
    if (!select) {
        console.error("Book select element (field-2) not found!");
        return;
    }
    try {
        const response = await fetch("/api/books/all");
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const books = await response.json();
        console.log("Books fetched:", books);
        select.innerHTML = '<option value="">Select one...</option>';
        books.filter(book => !book.is_deleted).forEach(book => {
            select.innerHTML += `<option value="${book.id}">${book.title}</option>`;
        });
    } catch (error) {
        console.error("Error fetching books:", error);
    }
}

// فیلتر کردن کاربران
async function filterUsers(searchTerm) {
    const select = document.getElementById("field-3");
    if (!select) {
        console.error("User select element not found!");
        return;
    }
    try {
        const response = await fetch("/api/users/all");
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const users = await response.json();
        select.innerHTML = '<option value="">Select one...</option>';
        const filtered = users
            .filter(user => !user.is_deleted)
            .filter(user =>
                (user.name || user.membership_id).toLowerCase().includes(searchTerm.toLowerCase())
            );
        filtered.forEach(user => {
            select.innerHTML += `<option value="${user.id}">${user.name || user.membership_id}</option>`;
        });
    } catch (error) {
        console.error("Error filtering users:", error);
    }
}

// فیلتر کردن کتاب‌ها
async function filterBooks(searchTerm) {
    const select = document.getElementById("field-2");
    if (!select) {
        console.error("Book select element not found!");
        return;
    }
    try {
        const response = await fetch("/api/books/all");
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const books = await response.json();
        select.innerHTML = '<option value="">Select one...</option>';
        const filtered = books
            .filter(book => !book.is_deleted)
            .filter(book =>
                book.title.toLowerCase().includes(searchTerm.toLowerCase())
            );
        filtered.forEach(book => {
            select.innerHTML += `<option value="${book.id}">${book.title}</option>`;
        });
    } catch (error) {
        console.error("Error filtering books:", error);
    }
}

// فیلتر کردن جدول با حفظ ردیف‌های انتخاب‌شده
function filterTable(searchTerm) {
    const tbody = document.querySelector(".giving-back-table tbody");
    if (!tbody) {
        console.error("Table body element not found!");
        return;
    }
    const checkboxes = document.querySelectorAll(".row-checkbox");
    const selectedNumbers = new Set(
        Array.from(checkboxes)
            .filter(cb => cb.checked)
            .map(cb => parseInt(cb.getAttribute("data-number")))
    );

    const rows = tbody.getElementsByTagName("tr");
    for (let row of rows) {
        const number = parseInt(row.querySelector(".row-checkbox").getAttribute("data-number"));
        const userName = row.cells[1].textContent.toLowerCase();
        const membership = row.cells[2].textContent.toLowerCase();
        const bookTitle = row.cells[3].textContent.toLowerCase();
        const matchesSearch = userName.includes(searchTerm.toLowerCase()) ||
                             membership.includes(searchTerm.toLowerCase()) ||
                             bookTitle.includes(searchTerm.toLowerCase());
        const isSelected = selectedNumbers.has(number);

        if (matchesSearch || isSelected) {
            row.style.display = "";
        } else {
            row.style.display = "none";
        }
    }
}

// ارسال انتخاب کاربر به سرور
async function submitBorrowing() {
    const userSelect = document.getElementById("field-3");
    const bookSelect = document.getElementById("field-2");
    const userId = userSelect.value;
    const bookId = bookSelect.value;

    if (!userId || !bookId) {
        alert("Please select both a person and a book!");
        return;
    }

    try {
        const response = await fetch(`/api/borrowing/borrow/${bookId}/${userId}`, {
            method: "GET"
        });
        
        const text = await response.text();
        console.log("Server response:", text);
        
        if (!response.ok) {
            throw new Error(text || "Failed to borrow the book. Please try again.");
        }
        
        // Clear selections
        userSelect.value = "";
        bookSelect.value = "";
        
        // Update both books list and table
        await Promise.all([
            populateBooks(),
            populateTable()
        ]);
        
        alert("Book borrowed successfully!");
    } catch (error) {
        console.error("Error borrowing book:", error);
        alert(error.message);
    }
}

function moveSelectedRowsToTop() {
    const tbody = document.querySelector(".giving-back-table tbody");
    if (!tbody) {
        console.error("Table body element not found!");
        return;
    }

    const rows = Array.from(tbody.getElementsByTagName("tr"));
    const selectedRows = rows.filter(row => row.querySelector(".row-checkbox").checked);
    const unselectedRows = rows.filter(row => !row.querySelector(".row-checkbox").checked);

    // پاک کردن همه ردیف‌ها
    rows.forEach(row => row.remove());

    // اضافه کردن ردیف‌های انتخاب شده به بالا
    selectedRows.forEach(row => tbody.appendChild(row));

    // اضافه کردن ردیف‌های انتخاب نشده به پایین
    unselectedRows.forEach(row => tbody.appendChild(row));
}

// رویدادها
document.addEventListener("DOMContentLoaded", () => {
    console.log("Script loaded and DOM fully loaded!");
    
    // تنظیم لینک‌های دکمه‌ها در صفحه اصلی
    const newBookButton = document.querySelector('.section-4:nth-child(1) a.button-2');
    if (newBookButton) {
        newBookButton.href = "/book/AddNew";
    }
    
    const newUserButton = document.querySelector('.section-4:nth-child(2) a.button-2');
    if (newUserButton) {
        newUserButton.href = "/user/new";
    }
    
    const newAuthorButton = document.querySelector('.section-4:nth-child(3) a.button-2');
    if (newAuthorButton) {
        newAuthorButton.href = "/author/new";
    }
    
    const newPublisherButton = document.querySelector('.section-4:nth-child(4) a.button-2');
    if (newPublisherButton) {
        newPublisherButton.href = "/publisher/new";
    }
    
    // Clear static table data
    const tbody = document.querySelector(".giving-back-table tbody");
    if (tbody) {
        tbody.innerHTML = "";
    }
    
    // Initial population of data
    populateUsers();
    populateBooks();
    populateTable();

    // Update borrow button
    const borrowButton = document.querySelector(".section-8 .submit-button");
    if (borrowButton) {
        borrowButton.textContent = "Borrow Book";
        borrowButton.className = "submit-button w-button";
        borrowButton.style.backgroundColor = "#3498db";
        borrowButton.style.color = "white";
        borrowButton.style.border = "none";
        borrowButton.style.padding = "10px 20px";
        borrowButton.style.cursor = "pointer";
        borrowButton.style.borderRadius = "15px";
        borrowButton.addEventListener("click", (e) => {
            e.preventDefault();
            submitBorrowing();
        });
    }

    // Update return button event listener
    const returnButton = document.querySelector('.return-button');
    if (returnButton) {
        returnButton.addEventListener('click', submitReturning);
    }
});

async function submitReturning() {
    const checkboxes = document.querySelectorAll(".row-checkbox:checked");
    if (checkboxes.length === 0) {
        alert("Please select at least one book to return!");
        return;
    }

    try {
        let hasError = false;
        const usersResponse = await fetch("/api/users/all");
        const users = await usersResponse.json();
        
        for (const checkbox of checkboxes) {
            const row = checkbox.closest("tr");
            const membershipIdFromRow = row.cells[2].textContent.trim();
            const bookTitle = row.cells[3].textContent.trim();
            
            const user = users.find(u => u.membership_id === membershipIdFromRow);
            if (!user) {
                console.error(`User with membership ID ${membershipIdFromRow} not found`);
                hasError = true;
                continue;
            }
            
            const booksResponse = await fetch("/api/books/all");
            const books = await booksResponse.json();
            const book = books.find(b => b.title.trim() === bookTitle);
            
            if (book) {
                const response = await fetch(`/api/borrowing/return/${book.id}/${user.id}`, {
                    method: "GET"
                });
                
                const text = await response.text();
                console.log("Return response:", text);
                
                if (!response.ok) {
                    hasError = true;
                    console.error(`Error returning book ${bookTitle}:`, text);
                }
            }
        }

        if (hasError) {
            throw new Error("Some books could not be returned. Please try again.");
        }

        await Promise.all([
            populateTable(),
            populateBooks()
        ]);

        alert("Books returned successfully!");
    } catch (error) {
        console.error("Error returning books:", error);
        alert(error.message);
    }
}


async function loadBooks() {
    const select = document.getElementById("field-2");
    select.innerHTML = '<option value="">Select a book</option>';
    try {
        const response = await fetch("/api/books");
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        const books = await response.json();
        books.forEach(book => {
            if (book.quantity && book.quantity > 0) {
                select.innerHTML += `<option value="${book.id}">${book.title} (Available: ${book.quantity})</option>`;
            }
        });
    } catch (error) {
        console.error("Error loading books:", error);
    }
}

async function populateTable() {
    const tbody = document.querySelector(".giving-back-table tbody");
    if (!tbody) {
        console.error("Table body element not found!");
        return;
    }
    tbody.innerHTML = "";

    try {
        // Get users list
        const usersResponse = await fetch("/api/users/all");
        if (!usersResponse.ok) {
            throw new Error(`HTTP error! status: ${usersResponse.status}`);
        }
        const users = await usersResponse.json();

        // Get books list
        const booksResponse = await fetch("/api/books/all");
        if (!booksResponse.ok) {
            throw new Error(`HTTP error! status: ${booksResponse.status}`);
        }
        const books = await booksResponse.json();

        // Create map for quick access to book information
        const booksMap = new Map(books.map(book => [book.id, book]));

        let rowNumber = 1;
        users.forEach(user => {
            // Only users who have borrowed books
            if (user.current_books && user.current_books.length > 0) {
                // Create a separate row for each borrowed book
                user.current_books.forEach(bookId => {
                    const book = booksMap.get(bookId);
                    if (book) {
                        const row = document.createElement("tr");
                        row.innerHTML = `
                            <td><input type="checkbox" class="row-checkbox" data-number="${rowNumber}"></td>
                            <td>${user.name || 'Unknown'}</td>
                            <td>${user.membership_id}</td>
                            <td>${book.title}</td>
                        `;
                        tbody.appendChild(row);
                        rowNumber++;
                    }
                });
            }
        });

        // Add event listeners for checkboxes
        const checkboxes = document.querySelectorAll(".row-checkbox");
        checkboxes.forEach(checkbox => {
            checkbox.addEventListener("change", moveSelectedRowsToTop);
        });

        // Update return button
        const returnButton = document.querySelector(".giving-back-table .submit-button");
        if (returnButton) {
            returnButton.textContent = "Return Selected Books";
            returnButton.className = "submit-button";
            returnButton.style.backgroundColor = "#3498db";
            returnButton.style.color = "white";
            returnButton.style.border = "none";
            returnButton.style.padding = "10px 20px";
            returnButton.style.cursor = "pointer";
            returnButton.style.borderRadius = "5px";
            returnButton.style.marginTop = "10px";
        }
    } catch (error) {
        console.error("Error populating table:", error);
    }
}
document.addEventListener('DOMContentLoaded', () => {
    // تغییر مسیر دکمه New Book
    const newBookButton = document.querySelector('a[href="/book/AddNew"]');
    if (newBookButton) {
        newBookButton.href = '/book/new';
    }
});