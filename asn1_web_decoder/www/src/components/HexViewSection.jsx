import { Box, Typography, Paper } from '@mui/material'
import HexView from './HexView'

function HexViewSection({ hexData, loading, highlightStart, highlightEnd }) {
  return (
    <Box sx={{ 
      display: 'flex', 
      flexDirection: 'column',
      height: '100%',
      overflow: 'hidden',
      flexShrink: 0,
      width: 'auto',
      maxWidth: 'max-content'
    }}>
      <Typography variant="h6" component="h2" fontWeight="bold" color="secondary" sx={{ mb: 1 }}>
        Hex View
      </Typography>
      
      <Paper 
        elevation={2}
        sx={{ 
          flexGrow: 1,
          p: 2,
          overflow: 'auto',
          bgcolor: '#fafafa',
          border: '2px solid',
          borderColor: 'secondary.light',
          display: 'flex',
          flexDirection: 'column'
        }}
      >
        <HexView 
          hexData={hexData}
          highlightStart={highlightStart}
          highlightEnd={highlightEnd}
        />
      </Paper>

      {hexData && (
        <Typography variant="caption" color="text.secondary" sx={{ mt: 1, textAlign: 'center' }}>
          Hover over tree elements to highlight hex bytes
        </Typography>
      )}
    </Box>
  )
}

export default HexViewSection
