document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('email-form');
    
    if (!form) {
        console.error('Form not found!');
        return;
    }

    console.log('Form found:', form);

    // اضافه کردن click event به دکمه submit با کلاس
    const submitButton = document.querySelector('.submit-button');
    if (!submitButton) {
        console.error('Submit button not found!');
        return;
    }

    // حذف حالت disabled از دکمه submit
    submitButton.removeAttribute('disabled');

    submitButton.addEventListener('click', async (e) => {
        e.preventDefault();
        console.log('Button clicked');
        
        const nameInput = document.getElementById('name');
        
        console.log('Input found:', {
            nameInput
        });
        
        if (!nameInput) {
            console.error('Form field not found:', {
                name: !nameInput
            });
            alert('Error: Form field not found');
            return;
        }
        
        const name = nameInput.value.trim();
        
        if (!name) {
            alert("Please fill the name field");
            return;
        }

        try {
            const response = await fetch('/api/publishers/create', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    name
                })
            });

            console.log('Response status:', response.status);
            const responseText = await response.text();
            console.log('Response text:', responseText);

            if (!response.ok) {
                throw new Error(responseText || 'Failed to create publisher');
            }

            alert('Publisher added successfully!');
            window.location.href = '/publisher/show-all';
        } catch (error) {
            console.error('Error:', error);
            alert(error.message);
        }
    });
});