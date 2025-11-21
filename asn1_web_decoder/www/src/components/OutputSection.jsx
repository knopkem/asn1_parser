import { Box, Typography, Paper, CircularProgress } from '@mui/material'
import TreeNode from './TreeNode'

function OutputSection({ decodedData, loading, onNodeHover }) {
  return (
    <Box sx={{ 
      display: 'flex', 
      flexDirection: 'column',
      height: '100%',
      overflow: 'hidden'
    }}>
      <Typography variant="h6" component="h2" fontWeight="bold" color="primary" sx={{ mb: 1 }}>
        Decoded Structure
      </Typography>
      
      <Paper 
        elevation={2}
        sx={{ 
          flexGrow: 1,
          p: 2,
          overflow: 'auto',
          bgcolor: '#ffffff',
          display: 'flex',
          flexDirection: 'column'
        }}
      >
        {loading && (
          <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', flexGrow: 1 }}>
            <CircularProgress />
            <Typography sx={{ mt: 2, color: 'primary.main' }}>
              Decoding...
            </Typography>
          </Box>
        )}
        
        {!loading && !decodedData && (
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', flexGrow: 1, textAlign: 'center', color: 'text.secondary' }}>
            <Typography variant="h6">
              Enter PEM data and click Decode to begin
            </Typography>
          </Box>
        )}
        
        {!loading && decodedData && (
          <Box>
            <TreeNode node={decodedData} onNodeHover={onNodeHover} />
          </Box>
        )}
      </Paper>
    </Box>
  )
}

export default OutputSection
