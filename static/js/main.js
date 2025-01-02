function toggleTheme() {
    const html = document.documentElement;
    const themeToggle = document.querySelector('.theme-toggle i');
    
    if (html.getAttribute('data-theme') === 'dark') {
        html.setAttribute('data-theme', 'light');
        themeToggle.className = 'fas fa-sun';
    } else {
        html.setAttribute('data-theme', 'dark');
        themeToggle.className = 'fas fa-moon';
    }
}

async function shortenUrl(event) {
    event.preventDefault();
    const urlInput = document.getElementById('crefax-url');
    const resultDiv = document.getElementById('result');
    const shortUrlP = document.getElementById('shortUrl');

    try {
        const response = await fetch('/api/shorten', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ url: urlInput.value }),
        });

        const data = await response.json();

        if (response.ok) {
            const baseUrl = `${window.location.protocol}//${window.location.host}`;
            const shortUrl = `${baseUrl}/${data.short_url.split('/').pop()}`;
            
            resultDiv.style.display = 'block';
            shortUrlP.textContent = shortUrl;
            resultDiv.style.animation = 'none';
            resultDiv.offsetHeight; // Reflow
            resultDiv.style.animation = null;
        } else {
            alert(data.error || 'Bir hata oluştu');
        }
    } catch (error) {
        alert('Bir hata oluştu: ' + error.message);
    }
}

function copyToClipboard() {
    const shortUrl = document.getElementById('shortUrl').textContent;
    navigator.clipboard.writeText(shortUrl).then(() => {
        const copyButton = document.querySelector('.copy-button');
        copyButton.textContent = 'Kopyalandı!';
        setTimeout(() => {
            copyButton.textContent = 'Kopyala';
        }, 2000);
    });
} 