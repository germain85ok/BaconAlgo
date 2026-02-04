# BaconAlgo UI Components

Core reusable UI components for the BaconAlgo SAAS platform built with Svelte 5 and TailwindCSS.

## Components

### 1. GlassCard
A glassmorphism container component with customizable padding and hover effects.

**Props:**
- `padding`: `'sm' | 'md' | 'lg' | 'xl'` (default: `'md'`)
- `hover`: `boolean` (default: `false`) - Enables hover elevation effect
- `class`: `string` - Additional CSS classes

**Example:**
```svelte
<GlassCard hover padding="lg">
  <h3>Card Title</h3>
  <p>Card content</p>
</GlassCard>
```

### 2. Modal
A modal dialog component with backdrop blur and smooth transitions.

**Props:**
- `isOpen`: `boolean` - Controls modal visibility
- `onClose`: `() => void` - Callback when modal should close
- `title`: `string` (optional) - Modal title
- `size`: `'sm' | 'md' | 'lg' | 'xl'` (default: `'md'`)

**Features:**
- Click outside to close
- Press Escape to close
- Smooth fade-in/out animations
- Prevents body scroll when open

**Example:**
```svelte
<script>
  let isOpen = $state(false);
</script>

<Modal {isOpen} onClose={() => isOpen = false} title="My Modal">
  <p>Modal content goes here</p>
</Modal>
```

### 3. Toast
A notification toast component with auto-dismiss.

**Props:**
- `message`: `string` - Toast message
- `type`: `'success' | 'error' | 'warning' | 'info'` (default: `'info'`)
- `duration`: `number` (default: `3000`) - Auto-dismiss time in ms (0 = no auto-dismiss)
- `onClose`: `() => void` (optional) - Callback when toast closes

**Example:**
```svelte
{#if showToast}
  <Toast 
    message="Action completed successfully!" 
    type="success" 
    duration={3000}
    onClose={() => showToast = false}
  />
{/if}
```

### 4. Navbar
Main navigation component with authentication support.

**Props:**
- `isLoggedIn`: `boolean` (default: `false`)
- `user`: `{ name: string; email: string } | null` (default: `null`)
- `onLogin`: `() => void` (optional)
- `onRegister`: `() => void` (optional)
- `onLogout`: `() => void` (optional)

**Features:**
- Sticky positioning
- Glassmorphism design
- Mobile responsive with hamburger menu
- User dropdown when authenticated

**Example:**
```svelte
<Navbar 
  isLoggedIn={true}
  user={{ name: 'John Doe', email: 'john@example.com' }}
  onLogout={() => handleLogout()}
/>
```

### 5. Footer
Footer component with branding and links.

**Props:**
- `class`: `string` (optional) - Additional CSS classes

**Features:**
- Social media icons (Discord, Twitter/X, GitHub)
- Company links
- Auto-updating copyright year

**Example:**
```svelte
<Footer />
```

## Installation

Components are already available in the project:

```svelte
import { GlassCard, Modal, Toast, Navbar, Footer } from '$lib/components/ui';
```

## Demo

Visit `/ui-demo` route to see all components in action.

## Design System

Components use the BaconAlgo color scheme:
- `bacon-orange`: #ff6b35
- `bacon-red`: #ff4d6a
- `success-cyan`: #00d9ff
- `warning-yellow`: #ffd93d
- `bg-dark`: #0a0a0f
- `text-primary`: #ffffff
- `text-secondary`: #a0a0a0

## Typography
- Display font: Orbitron
- Body font: Inter
- Mono font: JetBrains Mono
