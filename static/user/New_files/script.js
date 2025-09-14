document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('email-form');
    
    if (!form) {
        console.error('Form not found!');
        return;
    }

    console.log('Form found:', form);

    form.addEventListener('submit', async (e) => {
        e.preventDefault();
        console.log('Form submitted');
        
        // فقط دریافت نام و تلفن
        const nameInput = document.getElementById('name');
        const phoneInput = document.getElementById('name-2');
        
        console.log('Inputs found:', {
            nameInput,
            phoneInput
        });
        
        if (!nameInput || !phoneInput) {
            console.error('One or more form fields not found:', {
                name: !nameInput,
                phone: !phoneInput
            });
            alert('Error: Form fields not found');
            return;
        }
        
        const name = nameInput.value.trim();
        const phone = phoneInput.value.trim();
        
        if (!name || !phone) {
            alert("Please fill all fields");
            return;
        }

        // Generate unique membership_id
        const timestamp = new Date().getTime();
        const membership_id = `MEM${timestamp}`;

        try {
            const response = await fetch('/api/users/create', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    name,
                    phone,
                    membership_id
                })
            });

            console.log('Response status:', response.status);
            const responseText = await response.text();
            console.log('Response text:', responseText);

            if (!response.ok) {
                throw new Error(responseText || 'Failed to create user');
            }

            alert('User added successfully!');
            window.location.href = '/user/show-all';
        } catch (error) {
            console.error('Error:', error);
            alert(error.message);
        }
    });
});