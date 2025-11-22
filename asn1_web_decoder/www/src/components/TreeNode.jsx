import { useState } from 'react'
import { Box, IconButton, Typography, Snackbar, TextField } from '@mui/material'
import { ExpandMore, ChevronRight, ContentCopy, Edit, Check, Close } from '@mui/icons-material'

function TreeNode({ node, onNodeHover, onValueEdit }) {
  const [isCollapsed, setIsCollapsed] = useState(false)
  const [copySuccess, setCopySuccess] = useState(false)
  const [isEditing, setIsEditing] = useState(false)
  const [editValue, setEditValue] = useState('')
  const hasChildren = node.children && node.children.length > 0

  const toggleCollapse = (e) => {
    e.stopPropagation()
    setIsCollapsed(!isCollapsed)
  }

  const handleCopyValue = async (e) => {
    e.stopPropagation()
    if (node.value) {
      try {
        await navigator.clipboard.writeText(node.value)
        setCopySuccess(true)
      } catch (err) {
        console.error('Failed to copy:', err)
      }
    }
  }

  const handleStartEdit = (e) => {
    e.stopPropagation()
    setEditValue(node.value || '')
    setIsEditing(true)
  }

  const handleSaveEdit = (e) => {
    e.stopPropagation()
    if (onValueEdit) {
      onValueEdit(node, editValue)
    }
    setIsEditing(false)
  }

  const handleCancelEdit = (e) => {
    e.stopPropagation()
    setIsEditing(false)
    setEditValue('')
  }

  const handleEditChange = (e) => {
    setEditValue(e.target.value)
  }

  const handleCloseSnackbar = () => {
    setCopySuccess(false)
  }

  const handleMouseEnter = () => {
    if (onNodeHover) {
      onNodeHover(node)
    }
  }

  const handleMouseLeave = () => {
    if (onNodeHover) {
      onNodeHover(null)
    }
  }

  // Get color for tag class
  const getTagClassColor = (tagClass) => {
    switch (tagClass) {
      case 'UNIVERSAL':
        return '#1976d2' // Blue
      case 'APPLICATION':
        return '#d32f2f' // Red
      case 'CONTEXT':
        return '#388e3c' // Green
      case 'PRIVATE':
        return '#f57c00' // Orange
      case 'PEM':
        return '#7b1fa2' // Purple
      default:
        return '#616161' // Gray
    }
  }

  return (
    <Box sx={{ my: 0.2, fontFamily: '"Courier New", monospace' }}>
      <Box 
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
        sx={{ 
          display: 'flex',
          alignItems: 'flex-start',
          py: 0.25,
          px: 1,
          borderRadius: 1,
          cursor: 'pointer',
          transition: 'all 0.2s',
          '&:hover': {
            bgcolor: 'rgba(102, 126, 234, 0.12)',
            transform: 'translateX(2px)'
          }
        }}
      >
        {hasChildren ? (
          <IconButton 
            size="small" 
            onClick={toggleCollapse}
            sx={{ p: 0, mr: 0.5, width: 16, height: 16 }}
          >
            {isCollapsed ? <ChevronRight sx={{ fontSize: 14 }} /> : <ExpandMore sx={{ fontSize: 14 }} />}
          </IconButton>
        ) : (
          <Box sx={{ width: 24 }} />
        )}

        <Typography 
          component="span" 
          sx={{ 
            fontWeight: 600, 
            mr: 1,
            fontSize: '0.65rem',
            lineHeight: 1.2,
            color: getTagClassColor(node.tag_class)
          }}
        >
          {node.label.replace(/\s*\(Tag \d+\)/, '')}
        </Typography>

        <Typography 
          component="span" 
          sx={{ 
            color: 'text.secondary', 
            fontSize: '0.6rem',
            fontFamily: 'inherit',
            lineHeight: 1.2
          }}
        >
          (O: {node.byte_offset?.toString(16).padStart(4, '0') || '0000'}, L: {node.length})
        </Typography>

        {node.value && !isEditing && (
          <Box sx={{ display: 'flex', alignItems: 'flex-start', ml: 1, flex: 1, minWidth: 0 }}>
            <Typography 
              component="span" 
              sx={{ 
                color: '#0066cc', 
                fontSize: '0.6rem',
                fontFamily: 'inherit',
                lineHeight: 1.2,
                wordBreak: 'break-all',
                whiteSpace: 'normal',
                flex: 1,
                minWidth: 0
              }}
            >
              = {node.value}
            </Typography>
            <IconButton
              size="small"
              onClick={handleCopyValue}
              sx={{
                ml: 0.5,
                p: 0.25,
                opacity: 0.6,
                flexShrink: 0,
                '&:hover': {
                  opacity: 1,
                  bgcolor: 'rgba(0, 102, 204, 0.1)'
                }
              }}
              title="Copy value to clipboard"
            >
              <ContentCopy sx={{ fontSize: '0.75rem' }} />
            </IconButton>
            <IconButton
              size="small"
              onClick={handleStartEdit}
              sx={{
                ml: 0.25,
                p: 0.25,
                opacity: 0.6,
                flexShrink: 0,
                '&:hover': {
                  opacity: 1,
                  bgcolor: 'rgba(237, 108, 2, 0.1)'
                }
              }}
              title="Edit value"
            >
              <Edit sx={{ fontSize: '0.75rem' }} />
            </IconButton>
          </Box>
        )}

        {node.value && isEditing && (
          <Box sx={{ display: 'flex', alignItems: 'center', ml: 1, flex: 1, minWidth: 0 }}>
            <TextField
              value={editValue}
              onChange={handleEditChange}
              onClick={(e) => e.stopPropagation()}
              size="small"
              variant="outlined"
              sx={{
                flex: 1,
                minWidth: 0,
                '& .MuiInputBase-root': {
                  fontSize: '0.6rem',
                  fontFamily: '"Courier New", monospace',
                  lineHeight: 1.2,
                  py: 0.25,
                  px: 0.5
                },
                '& .MuiInputBase-input': {
                  py: 0.25,
                  px: 0.5
                }
              }}
              autoFocus
            />
            <IconButton
              size="small"
              onClick={handleSaveEdit}
              sx={{
                ml: 0.5,
                p: 0.25,
                opacity: 0.6,
                flexShrink: 0,
                color: 'success.main',
                '&:hover': {
                  opacity: 1,
                  bgcolor: 'rgba(46, 125, 50, 0.1)'
                }
              }}
              title="Save changes"
            >
              <Check sx={{ fontSize: '0.75rem' }} />
            </IconButton>
            <IconButton
              size="small"
              onClick={handleCancelEdit}
              sx={{
                ml: 0.25,
                p: 0.25,
                opacity: 0.6,
                flexShrink: 0,
                color: 'error.main',
                '&:hover': {
                  opacity: 1,
                  bgcolor: 'rgba(211, 47, 47, 0.1)'
                }
              }}
              title="Cancel editing"
            >
              <Close sx={{ fontSize: '0.75rem' }} />
            </IconButton>
          </Box>
        )}
      </Box>

      {hasChildren && !isCollapsed && (
        <Box sx={{ 
          ml: 2.5,
          pl: 1,
          borderLeft: '1px solid #ddd'
        }}>
          {node.children.map((child, index) => (
            <TreeNode key={index} node={child} onNodeHover={onNodeHover} onValueEdit={onValueEdit} />
          ))}
        </Box>
      )}

      <Snackbar
        open={copySuccess}
        autoHideDuration={2000}
        onClose={handleCloseSnackbar}
        message="Value copied to clipboard"
        anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
      />
    </Box>
  )
}

export default TreeNode
