import { useState, useEffect } from 'react'
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { CssBaseline, Box, Typography, Fab, AppBar, Toolbar } from '@mui/material'
import { Edit } from '@mui/icons-material'
import InputDialog from './components/InputDialog'
import OutputSection from './components/OutputSection'
import HexViewSection from './components/HexViewSection'
import init, { decode_pem_to_json, pem_to_hex, encode_tree_to_pem } from './wasm/asn1_web_decoder.js'
import wasmUrl from './wasm/asn1_web_decoder_bg.wasm?url'

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
-----END CERTIFICATE-----`

const theme = createTheme({
  palette: {
    primary: {
      main: '#667eea',
    },
    secondary: {
      main: '#764ba2',
    },
  },
})

function App() {
  const [input, setInput] = useState('')
  const [originalPemLabel, setOriginalPemLabel] = useState('CERTIFICATE')
  const [decodedData, setDecodedData] = useState(null)
  const [modifiedData, setModifiedData] = useState(null)
  const [hexData, setHexData] = useState('')
  const [error, setError] = useState('')
  const [loading, setLoading] = useState(false)
  const [wasmReady, setWasmReady] = useState(false)
  const [highlightRange, setHighlightRange] = useState({ start: null, end: null })
  const [dialogOpen, setDialogOpen] = useState(true)

  useEffect(() => {
    init(wasmUrl)
      .then(() => {
        console.log('WASM module initialized successfully')
        setWasmReady(true)
      })
      .catch(err => {
        console.error('Failed to initialize WASM module:', err)
        setError(`Failed to initialize WASM module: ${err.message || err}`)
      })
  }, [])

  const handleDecode = async () => {
    if (!input.trim()) {
      setError('Please enter PEM-formatted ASN.1 data')
      return
    }

    setError('')
    setLoading(true)
    setDecodedData(null)
    setModifiedData(null)
    setHexData('')
    setDialogOpen(false)

    try {
      const result = decode_pem_to_json(input)
      const hex = pem_to_hex(input)
      const data = JSON.parse(result)
      
      // Extract PEM label from input
      const pemMatch = input.match(/-----BEGIN ([A-Z\s]+)-----/)
      if (pemMatch) {
        setOriginalPemLabel(pemMatch[1])
      }
      
      setDecodedData(data)
      setModifiedData(JSON.parse(JSON.stringify(data))) // Deep copy
      setHexData(hex)
    } catch (e) {
      setError(e.toString())
      setDialogOpen(true)
    } finally {
      setLoading(false)
    }
  }

  const handleClear = () => {
    setInput('')
    setDecodedData(null)
    setModifiedData(null)
    setHexData('')
    setError('')
    setHighlightRange({ start: null, end: null })
  }

  const handleLoadSample = () => {
    setInput(SAMPLE_CERT)
    setError('')
  }

  const handleNodeHover = (node) => {
    if (node && node.byte_offset !== undefined) {
      setHighlightRange({
        start: node.byte_offset,
        end: node.byte_offset + node.byte_length
      })
    } else {
      setHighlightRange({ start: null, end: null })
    }
  }

  const handleValueEdit = (node, newValue) => {
    // Update the modified data tree
    const updateNodeValue = (treeNode) => {
      if (treeNode === node) {
        treeNode.value = newValue
        return true
      }
      if (treeNode.children) {
        for (const child of treeNode.children) {
          if (updateNodeValue(child)) {
            return true
          }
        }
      }
      return false
    }

    const updatedData = JSON.parse(JSON.stringify(modifiedData))
    updateNodeValue(updatedData)
    setModifiedData(updatedData)

    // Encode the modified tree back to PEM
    try {
      const jsonTree = JSON.stringify(updatedData)
      const newPem = encode_tree_to_pem(jsonTree, originalPemLabel)
      
      // Update the input with the new PEM
      setInput(newPem)
      
      // Decode the new PEM to update hex view
      const hex = pem_to_hex(newPem)
      setHexData(hex)
      
      console.log('Successfully encoded modified ASN.1 to PEM')
    } catch (e) {
      console.error('Failed to encode ASN.1:', e)
      setError(`Encoding failed: ${e.toString()}`)
    }
  }

  const handleKeyDown = (e) => {
    if (e.ctrlKey && e.key === 'Enter') {
      handleDecode()
    }
  }

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box sx={{ 
        height: '100vh',
        display: 'flex',
        flexDirection: 'column',
        overflow: 'hidden'
      }}>
        {/* App Bar */}
        <AppBar position="static" sx={{ 
          background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)'
        }}>
          <Toolbar>
            <Typography variant="h6" component="h1" sx={{ flexGrow: 1, fontWeight: 'bold' }}>
              ASN.1 PEM Decoder
            </Typography>
            <Typography variant="caption" sx={{ opacity: 0.9 }}>
              Hover over tree nodes to highlight hex bytes
            </Typography>
          </Toolbar>
        </AppBar>

        {/* Main Content */}
        <Box sx={{ 
          flexGrow: 1,
          display: 'flex',
          gap: 2,
          p: 2,
          overflow: 'hidden',
          bgcolor: '#f5f5f5'
        }}>
          <HexViewSection
            hexData={hexData}
            loading={loading}
            highlightStart={highlightRange.start}
            highlightEnd={highlightRange.end}
          />

          <OutputSection
            decodedData={decodedData}
            loading={loading}
            onNodeHover={handleNodeHover}
            onValueEdit={handleValueEdit}
          />
        </Box>

        {/* Floating Action Button */}
        {!dialogOpen && decodedData && (
          <Fab 
            color="primary" 
            aria-label="edit input"
            onClick={() => setDialogOpen(true)}
            sx={{ 
              position: 'fixed',
              bottom: 16,
              right: 16,
              background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
            }}
          >
            <Edit />
          </Fab>
        )}

        {/* Input Dialog */}
        <InputDialog
          open={dialogOpen}
          input={input}
          error={error}
          wasmReady={wasmReady}
          onInputChange={setInput}
          onDecode={handleDecode}
          onClear={handleClear}
          onLoadSample={handleLoadSample}
          onClose={() => decodedData && setDialogOpen(false)}
        />
      </Box>
    </ThemeProvider>
  )
}

export default App
