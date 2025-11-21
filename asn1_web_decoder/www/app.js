import init, { decode_pem_to_json } from './pkg/asn1_web_decoder.js';

const SAMPLE_CERT = `-----BEGIN CERTIFICATE-----
MIIDxzCCAq+gAwIBAgIUQ0cPkEzTGcHIPhQzgECtTTYVHSgwDQYJKoZIhvcNAQEL
BQAwczELMAkGA1UEBhMCVVMxEzARBgNVBAgMCkNhbGlmb3JuaWExFjAUBgNVBAcM
DVNhbiBGcmFuY2lzY28xITAfBgNVBAoMGEludGVybmV0IFdpZGdldHMgUHR5IEx0
ZDEUMBIGA1UEAwwLZXhhbXBsZS5jb20wHhcNMjUxMTIxMTIyMjExWhcNMjYxMTIx
MTIyMjExWjBzMQswCQYDVQQGEwJVUzETMBEGA1UECAwKQ2FsaWZvcm5pYTEWMBQG
A1UEBwwNU2FuIEZyYW5jaXNjbzEhMB8GA1UECgwYSW50ZXJuZXQgV2lkZ2V0cyBQ
dHkgTHRkMRQwEgYDVQQDDAtleGFtcGxlLmNvbTCCASIwDQYJKoZIhvcNAQEBBQAD
ggEPADCCAQoCggEBANOSLr1SqcmhwAVpZzYt1XEuOLEzrIGxY6D9qPvyI7mpCHMj
Q2uLiqd+Fia2wdAlJXvGO+nkyYDbgZ1zZOjTlmqw2sz9g4IpyqS2diuq2EX1Hmyp
dRnxinvsW2a9SNuhEsZKxlwsKBot0e5mSq8lyrUZVA+yewC5beUdJPmZdZPtW4B6
8aPX/Lh8v+oSutxomK5M8O22sja+AtPxgwkp7jDwJSOV0aE9aWjY+0yHl2NjiyOx
6Cd7mHseZnikcPhdiWFbJdJF7YMq//Xl7gm66Sf7FvqjZHm3XAxITtHqiaiUL8DG
S05zz8eezohyHocqlGJX57tFDjI4afXysiFW8X8CAwEAAaNTMFEwHQYDVR0OBBYE
FMIKfH9+g5UmLcqhehuWzS2Ow9uoMB8GA1UdIwQYMBaAFMIKfH9+g5UmLcqhehuW
zS2Ow9uoMA8GA1UdEwEB/wQFMAMBAf8wDQYJKoZIhvcNAQELBQADggEBACat7jU0
luyWv3Qli7h1eTinyYwgWH+TYreGMakyOSfT9p4jmlB5WrxMESSEvzwuAVhLbJsH
HIOiOVcviUPdPkfw5rgLoaDyy4Yd0LGpnkH4iT9TEZ80O091Bzd64poZf1PS2lUy
HX+0yeyPOqag2kxNO6PU5gP34K9sAeF4/ectOtC1sa/EJ2ukyR2LxGYp0/N0tCm4
0kUTi6kJ/Po6a1cWRPTaxyWRdXm6160RKaTU5JTlZ/aQckDSeUe8LAjAjCUF+f+8
E2KZ0OXNoPWlpGKr5tO0A/pwGU5ILNb2wERDbfZv7ljCLz8UUGu1kLSrTQovZbIN
Vi8m3NapLgvi9QM=
-----END CERTIFICATE-----`;

let wasmInitialized = false;

async function initWasm() {
    if (!wasmInitialized) {
        await init();
        wasmInitialized = true;
    }
}

function showError(message) {
    const errorDiv = document.getElementById('error');
    errorDiv.textContent = message;
    errorDiv.classList.add('show');
}

function hideError() {
    const errorDiv = document.getElementById('error');
    errorDiv.classList.remove('show');
}

function createTreeNode(node, depth = 0) {
    const nodeDiv = document.createElement('div');
    nodeDiv.className = 'tree-node';
    
    const contentDiv = document.createElement('div');
    contentDiv.className = 'tree-node-content';
    
    const hasChildren = node.children && node.children.length > 0;
    
    if (hasChildren) {
        const toggleBtn = document.createElement('button');
        toggleBtn.className = 'tree-node-toggle';
        toggleBtn.innerHTML = '▼';
        toggleBtn.onclick = (e) => {
            e.stopPropagation();
            const childrenDiv = nodeDiv.querySelector('.tree-node-children');
            if (childrenDiv) {
                childrenDiv.classList.toggle('collapsed');
                toggleBtn.innerHTML = childrenDiv.classList.contains('collapsed') ? '▶' : '▼';
            }
        };
        contentDiv.appendChild(toggleBtn);
    } else {
        const emptyToggle = document.createElement('span');
        emptyToggle.className = 'tree-node-toggle empty';
        contentDiv.appendChild(emptyToggle);
    }
    
    const labelSpan = document.createElement('span');
    labelSpan.className = 'tree-node-label';
    labelSpan.textContent = node.label;
    contentDiv.appendChild(labelSpan);
    
    const classBadge = document.createElement('span');
    classBadge.className = `badge badge-${node.tag_class.toLowerCase()}`;
    classBadge.textContent = node.tag_class;
    contentDiv.appendChild(classBadge);
    
    if (node.tag_class !== 'PEM') {
        const typeBadge = document.createElement('span');
        typeBadge.className = `badge badge-${node.is_constructed ? 'constructed' : 'primitive'}`;
        typeBadge.textContent = node.is_constructed ? 'CONSTRUCTED' : 'PRIMITIVE';
        contentDiv.appendChild(typeBadge);
    }
    
    const infoSpan = document.createElement('span');
    infoSpan.className = 'tree-node-info';
    infoSpan.textContent = `[Length: ${node.length}]`;
    contentDiv.appendChild(infoSpan);
    
    if (node.value) {
        const valueSpan = document.createElement('span');
        valueSpan.className = 'tree-node-value';
        valueSpan.textContent = `= ${node.value}`;
        contentDiv.appendChild(valueSpan);
    }
    
    nodeDiv.appendChild(contentDiv);
    
    if (hasChildren) {
        const childrenDiv = document.createElement('div');
        childrenDiv.className = 'tree-node-children';
        
        for (const child of node.children) {
            childrenDiv.appendChild(createTreeNode(child, depth + 1));
        }
        
        nodeDiv.appendChild(childrenDiv);
    }
    
    return nodeDiv;
}

function renderTree(jsonData) {
    const treeDiv = document.getElementById('tree');
    treeDiv.innerHTML = '';
    
    try {
        const data = JSON.parse(jsonData);
        treeDiv.appendChild(createTreeNode(data));
    } catch (e) {
        showError(`Failed to parse decoded data: ${e.message}`);
    }
}

async function decodePem() {
    const input = document.getElementById('pemInput').value.trim();
    
    if (!input) {
        showError('Please enter PEM-formatted ASN.1 data');
        return;
    }
    
    hideError();
    
    const treeDiv = document.getElementById('tree');
    treeDiv.innerHTML = '<div class="loading">Decoding</div>';
    
    try {
        await initWasm();
        const result = decode_pem_to_json(input);
        renderTree(result);
    } catch (e) {
        showError(e.toString());
        treeDiv.innerHTML = '<div class="empty-state">Failed to decode. Please check your input.</div>';
    }
}

function clearInput() {
    document.getElementById('pemInput').value = '';
    document.getElementById('tree').innerHTML = '<div class="empty-state">Paste PEM data and click Decode</div>';
    hideError();
}

function loadSample() {
    document.getElementById('pemInput').value = SAMPLE_CERT;
    hideError();
}

document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('decodeBtn').addEventListener('click', decodePem);
    document.getElementById('clearBtn').addEventListener('click', clearInput);
    document.getElementById('sampleBtn').addEventListener('click', loadSample);
    
    document.getElementById('tree').innerHTML = '<div class="empty-state">Paste PEM data and click Decode</div>';
    
    document.getElementById('pemInput').addEventListener('keydown', (e) => {
        if (e.ctrlKey && e.key === 'Enter') {
            decodePem();
        }
    });
});
