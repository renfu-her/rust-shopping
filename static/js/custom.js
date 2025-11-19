// Custom JavaScript

$(document).ready(function() {
    // Auto-hide alerts after 5 seconds
    setTimeout(function() {
        $('.alert').fadeOut('slow');
    }, 5000);
    
    // Form validation
    $('form').on('submit', function(e) {
        let isValid = true;
        $(this).find('input[required], select[required], textarea[required]').each(function() {
            if (!$(this).val()) {
                isValid = false;
                $(this).addClass('is-invalid');
            } else {
                $(this).removeClass('is-invalid');
            }
        });
        
        if (!isValid) {
            e.preventDefault();
            alert('請填寫所有必填欄位');
        }
    });
    
    // Remove invalid class on input
    $('input, select, textarea').on('input change', function() {
        $(this).removeClass('is-invalid');
    });
});

