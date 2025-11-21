import { Box, Typography, TextField, Button, Alert, Stack } from '@mui/material'
import { PlayArrow, Clear, CloudDownload } from '@mui/icons-material'

function InputSection({ input, error, wasmReady, onInputChange, onDecode, onClear, onLoadSample, onKeyDown }) {
  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
      <Typography variant="h5" component="h2" fontWeight="bold" color="primary">
        Input
      </Typography>
      
      <TextField
        multiline
        minRows={15}
        maxRows={20}
        value={input}
        onChange={(e) => onInputChange(e.target.value)}
        onKeyDown={onKeyDown}
        placeholder={`-----BEGIN CERTIFICATE-----
MIIDXTCCAkWgAwIBAgIJAKVpbF9K...
-----END CERTIFICATE-----`}
        variant="outlined"
        fullWidth
        sx={{
          '& .MuiInputBase-root': {
            fontFamily: '"Courier New", monospace',
            fontSize: '0.875rem'
          }
        }}
      />
      
      <Stack direction="row" spacing={1}>
        <Button 
          variant="contained"
          color="primary"
          startIcon={<PlayArrow />}
          onClick={onDecode}
          disabled={!wasmReady}
          fullWidth
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
          startIcon={<Clear />}
          onClick={onClear}
        >
          Clear
        </Button>
        
        <Button 
          variant="outlined"
          startIcon={<CloudDownload />}
          onClick={onLoadSample}
        >
          Sample
        </Button>
      </Stack>
      
      {error && (
        <Alert severity="error" onClose={() => {}}>
          {error}
        </Alert>
      )}
    </Box>
  )
}

export default InputSection
