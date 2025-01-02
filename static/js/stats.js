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

// URL'den kısa kodu al ve istatistikleri yükle
function loadStats() {
    const shortCode = window.location.pathname.slice(1, -1);
    const baseUrl = `${window.location.protocol}//${window.location.host}`;
    
    fetch(`/api/stats/${shortCode}`)
        .then(response => response.json())
        .then(data => {
            document.getElementById('shortUrl').textContent = `${baseUrl}/${shortCode}`;
            document.getElementById('longUrl').textContent = data.long_url;
            document.getElementById('clicks').textContent = data.clicks;
            document.getElementById('createdAt').textContent = new Date(data.created_at).toLocaleString('tr-TR');
        })
        .catch(error => {
            console.error('Hata:', error);
            alert('Veriler yüklenirken bir hata oluştu.');
        });
}

// Sayfa yüklendiğinde istatistikleri yükle
document.addEventListener('DOMContentLoaded', loadStats); 