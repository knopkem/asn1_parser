import { Dialog, DialogTitle, DialogContent, DialogActions, TextField, Button, Alert, Stack, IconButton } from '@mui/material'
import { PlayArrow, Clear, CloudDownload, Close } from '@mui/icons-material'

function InputDialog({ open, input, error, wasmReady, onInputChange, onDecode, onClear, onLoadSample, onClose }) {
  const handleKeyDown = (e) => {
    if (e.ctrlKey && e.key === 'Enter') {
      onDecode()
    }
  }

  return (
    <Dialog 
      open={open} 
      onClose={onClose}
      maxWidth="md"
      fullWidth
      PaperProps={{
        sx: {
          minHeight: '70vh'
        }
      }}
    >
      <DialogTitle sx={{ 
        background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
        color: 'white',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center'
      }}>
        Enter PEM Data
        {onClose && (
          <IconButton
            edge="end"
            color="inherit"
            onClick={onClose}
            aria-label="close"
            sx={{ ml: 2 }}
          >
            <Close />
          </IconButton>
        )}
      </DialogTitle>
      
      <DialogContent sx={{ pt: 3 }}>
        <TextField
          multiline
          minRows={20}
          maxRows={25}
          value={input}
          onChange={(e) => onInputChange(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={`Paste your PEM-formatted ASN.1 data here...

Example:
-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKVpbF9K...
-----END CERTIFICATE-----

Press Ctrl+Enter to decode`}
          variant="outlined"
          fullWidth
          autoFocus
          sx={{
            '& .MuiInputBase-root': {
              fontFamily: '"Courier New", monospace',
              fontSize: '0.875rem'
            }
          }}
        />
        
        {error && (
          <Alert severity="error" sx={{ mt: 2 }}>
            {error}
          </Alert>
        )}
      </DialogContent>
      
      <DialogActions sx={{ px: 3, pb: 3 }}>
        <Stack direction="row" spacing={1} sx={{ width: '100%' }}>
          <Button 
            variant="contained"
            color="primary"
            startIcon={<PlayArrow />}
            onClick={onDecode}
            disabled={!wasmReady}
            fullWidth
            size="large"
            sx={{ 
              background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
              '&:hover': {
                background: 'linear-gradient(135deg, #5568d3 0%, #653a8b 100%)',
              }
            }}
          >
            {wasmReady ? 'Decode' : 'Loading...'}
          </Button>
          
          <Button 
            variant="outlined"
            startIcon={<CloudDownload />}
            onClick={onLoadSample}
            size="large"
          >
            Sample
          </Button>
          
          <Button 
            variant="outlined"
            startIcon={<Clear />}
            onClick={onClear}
            size="large"
          >
            Clear
          </Button>
        </Stack>
      </DialogActions>
    </Dialog>
  )
}

export default InputDialog
