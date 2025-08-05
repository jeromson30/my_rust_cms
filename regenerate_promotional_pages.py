#!/usr/bin/env python3
"""
Regenerate Promotional Pages with Improved Components
Deletes old promotional pages and recreates them with the new component system.
"""

import requests
import json
import time
import uuid
from typing import Dict, Any, List

# API Configuration
API_BASE = "http://localhost:8081/api"
LOGIN_URL = f"{API_BASE}/auth/login"
PAGES_URL = f"{API_BASE}/pages"

# Default admin credentials
ADMIN_CREDENTIALS = {
    "username": "admin",
    "password": "admin"
}

def authenticate() -> str:
    """Authenticate and get a session token."""
    try:
        response = requests.post(LOGIN_URL, json=ADMIN_CREDENTIALS)
        if response.status_code == 200:
            data = response.json()
            return data.get("token", "")
        else:
            print(f"❌ Authentication failed: {response.status_code}")
            return ""
    except Exception as e:
        print(f"❌ Authentication error: {e}")
        return ""

def delete_page(token: str, page_id: int) -> bool:
    """Delete a page by ID."""
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json"
    }
    
    try:
        response = requests.delete(f"{PAGES_URL}/{page_id}", headers=headers)
        if response.status_code == 204:
            print(f"✅ Deleted page ID: {page_id}")
            return True
        else:
            print(f"❌ Failed to delete page {page_id}: {response.status_code}")
            return False
    except Exception as e:
        print(f"❌ Error deleting page {page_id}: {e}")
        return False

def create_page_component(component_type: str, content: str, styles: Dict = None, properties: Dict = None) -> Dict:
    """Create a PageComponent object with improved styling."""
    default_styles = {
        "background_color": "transparent",
        "text_color": "var(--public-text-primary, #000000)",
        "padding": "16px",
        "margin": "8px",
        "border_radius": "8px",
        "font_size": "16px",
        "font_weight": "normal",
        "text_align": "left",
        "border_width": "0px",
        "border_color": "transparent",
        "border_style": "solid",
        "box_shadow": "none",
        "opacity": 1.0,
        "z_index": 1,
        "font_family": "system-ui, -apple-system, sans-serif",
        "line_height": "1.5",
        "letter_spacing": "normal",
        "text_decoration": "none",
        "text_transform": "none",
        "background_image": "none",
        "background_size": "cover",
        "background_position": "center",
        "background_repeat": "no-repeat"
    }
    
    if styles:
        default_styles.update(styles)
    
    default_properties = {
        "image_url": "",
        "image_alt": "",
        "image_title": "",
        "image_lazy_load": False,
        "button_text": "",
        "button_url": "",
        "button_target": "_self",
        "button_size": "medium",
        "button_variant": "primary",
        "button_icon": "",
        "form_action": "",
        "form_method": "POST",
        "form_fields": [],
        "video_url": "",
        "video_autoplay": False,
        "video_controls": True,
        "video_muted": False,
        "video_loop": False,
        "gallery_images": [],
        "gallery_layout": "grid",
        "gallery_columns": 3,
        "list_type": "unordered",
        "list_items": [],
        "container_max_width": "1200px",
        "container_align": "center",
        "divider_style": "solid",
        "divider_thickness": "1px",
        "divider_color": "#e1e5e9",
        "divider_margin": "20px",
        "divider_width": "100%",
        "animation_type": "none",
        "animation_duration": "0.3s",
        "animation_delay": "0s",
        "seo_title": "",
        "seo_description": "",
        "seo_keywords": [],
        "aria_label": "",
        "aria_description": "",
        "tab_index": 0
    }
    
    if properties:
        default_properties.update(properties)
    
    return {
        "id": str(uuid.uuid4()),
        "component_type": component_type,
        "content": content,
        "styles": default_styles,
        "position": {
            "x": 0.0,
            "y": 0.0,
            "width": "100%",
            "height": "auto"
        },
        "properties": default_properties
    }

def get_improved_about_cms_components() -> List[Dict]:
    """Generate improved components for the About My Rust CMS page."""
    components = []
    
    # Hero Section with dramatic gradient
    components.append(create_page_component(
        "Hero",
        "Welcome to My Rust CMS",
        {
            "background_color": "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            "text_color": "white",
            "padding": "100px 40px",
            "text_align": "center",
            "font_size": "56px",
            "font_weight": "bold",
            "margin": "0px",
            "border_radius": "0px"
        }
    ))
    
    # Subtitle with better spacing
    components.append(create_page_component(
        "Subheading",
        "The Revolutionary Content Management System Built with the RAYDT Stack",
        {
            "text_align": "center",
            "font_size": "28px",
            "color": "#2c3e50",
            "margin": "40px auto",
            "max_width": "800px",
            "line_height": "1.4",
            "font_weight": "300"
        }
    ))
    
    # Introduction paragraph with better typography
    components.append(create_page_component(
        "Text",
        "Experience the perfect fusion of blazing performance, memory safety, and developer productivity. Built entirely with Rust across the full stack, My Rust CMS represents a paradigm shift in web development.",
        {
            "font_size": "20px",
            "line_height": "1.7",
            "margin": "40px auto",
            "max_width": "700px",
            "text_align": "center",
            "color": "#555"
        }
    ))
    
    # Spacer
    components.append(create_page_component("Spacer", "60px"))
    
    # Why Choose Section with improved styling
    components.append(create_page_component(
        "Heading",
        "🌟 Why Choose My Rust CMS?",
        {
            "font_size": "42px",
            "margin": "60px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # Three-column feature showcase
    components.append(create_page_component(
        "ThreeColumn",
        "⚡ **Unmatched Performance**\n\n• WebAssembly frontend delivers near-native performance\n• Zero-cost abstractions ensure maximum efficiency\n• Sub-millisecond response times handle thousands of concurrent requests\n\n🔒 **Enterprise-Grade Security**\n\n• Zero buffer overflows or memory leaks\n• Compile-time bug prevention\n• Built-in protection against web vulnerabilities\n\n🛠️ **Superior Developer Experience**\n\n• Single language across entire stack\n• \"If it compiles, it works\" reliability\n• Modern tooling with testing and documentation",
        {
            "margin": "40px 0px",
            "padding": "0px"
        }
    ))
    
    # Spacer
    components.append(create_page_component("Spacer", "60px"))
    
    # Perfect For Section
    components.append(create_page_component(
        "Heading",
        "🎯 Perfect For",
        {
            "font_size": "42px",
            "margin": "60px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # Two Column Layout for use cases
    components.append(create_page_component(
        "TwoColumn",
        "🏢 **Enterprise Applications**\nRequiring maximum reliability and performance for mission-critical systems.\n\n🌐 **High-Traffic Websites**\nNeeding blazing-fast response times and efficient resource utilization.\n\n🔐 **Security-Critical Systems**\nWhere memory safety and type safety are absolutely paramount.\n\n⚡ **Modern Development Teams**\nEmbracing cutting-edge technology and best practices.",
        {
            "margin": "40px 0px",
            "padding": "20px"
        }
    ))
    
    # Spacer
    components.append(create_page_component("Spacer", "80px"))
    
    # Call-to-Action Section
    components.append(create_page_component(
        "Heading",
        "🚀 Ready to Experience the Future?",
        {
            "font_size": "38px",
            "margin": "60px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # CTA Buttons with improved styling
    components.append(create_page_component(
        "Button",
        "Explore Features & Capabilities",
        {
            "background_color": "#007bff",
            "text_color": "white",
            "padding": "18px 36px",
            "border_radius": "12px",
            "font_size": "20px",
            "font_weight": "bold",
            "margin": "20px auto",
            "text_align": "center",
            "box_shadow": "0 4px 12px rgba(0,123,255,0.3)",
            "transition": "all 0.3s ease"
        },
        {
            "button_text": "Explore Features & Capabilities",
            "button_url": "/page/features-capabilities",
            "button_target": "_self",
            "button_variant": "primary"
        }
    ))
    
    components.append(create_page_component(
        "Button",
        "Learn About RAYDT Stack",
        {
            "background_color": "#28a745",
            "text_color": "white",
            "padding": "18px 36px",
            "border_radius": "12px",
            "font_size": "20px",
            "font_weight": "bold",
            "margin": "20px auto",
            "text_align": "center",
            "box_shadow": "0 4px 12px rgba(40,167,69,0.3)",
            "transition": "all 0.3s ease"
        },
        {
            "button_text": "Learn About RAYDT Stack",
            "button_url": "/page/raydt-stack-technology",
            "button_target": "_self",
            "button_variant": "secondary"
        }
    ))
    
    return components

def get_improved_raydt_stack_components() -> List[Dict]:
    """Generate improved components for the RAYDT Stack Technology page."""
    components = []
    
    # Hero Section
    components.append(create_page_component(
        "Hero",
        "🚀 The RAYDT Stack Revolution",
        {
            "background_color": "linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%)",
            "text_color": "white",
            "padding": "100px 40px",
            "text_align": "center",
            "font_size": "56px",
            "font_weight": "bold",
            "margin": "0px",
            "border_radius": "0px"
        }
    ))
    
    # Subtitle
    components.append(create_page_component(
        "Subheading",
        "Rust • Axum • Yew • Diesel • Tower",
        {
            "text_align": "center",
            "font_size": "32px",
            "color": "#e74c3c",
            "margin": "40px auto",
            "font_weight": "bold",
            "letter_spacing": "2px"
        }
    ))
    
    # Introduction
    components.append(create_page_component(
        "Text",
        "The world's first production-ready full-stack web development paradigm built entirely in Rust. Experience unprecedented memory safety, blazing performance, and fearless concurrency across every layer.",
        {
            "font_size": "20px",
            "line_height": "1.7",
            "margin": "40px auto",
            "max_width": "800px",
            "text_align": "center",
            "color": "#555"
        }
    ))
    
    # Spacer
    components.append(create_page_component("Spacer", "60px"))
    
    # Architecture Section
    components.append(create_page_component(
        "Heading",
        "🏗️ Revolutionary Architecture",
        {
            "font_size": "42px",
            "margin": "60px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # Architecture Visual as a Card
    components.append(create_page_component(
        "Card",
        "Stack Layers\n\n🎨 **YEW Frontend**\nWebAssembly • Component-Based • Reactive\n\n⚡ **AXUM Backend** \nAsync-First • Type-Safe • High-Performance\n\n🔒 **TOWER Middleware**\nService Architecture • Composable • Modular\n\n🗃️ **DIESEL ORM**\nType-Safe Queries • Migration • Schema\n\n🦀 **RUST Foundation**\nMemory Safety • Zero-Cost Abstractions • Speed",
        {
            "background_color": "#f8f9fa",
            "border": "3px solid #e74c3c",
            "border_radius": "16px",
            "padding": "40px",
            "margin": "40px auto",
            "text_align": "center",
            "font_family": "monospace",
            "font_size": "18px",
            "line_height": "2.2",
            "max_width": "600px",
            "box_shadow": "0 8px 24px rgba(0,0,0,0.1)"
        }
    ))
    
    # Component Deep Dive
    components.append(create_page_component(
        "Heading",
        "⚡ Component Deep Dive",
        {
            "font_size": "42px",
            "margin": "80px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # Five component cards in a responsive layout using individual cards
    components.append(create_page_component(
        "Card",
        "🦀 Rust - The Foundation\n\n**Memory Safety**: Eliminates entire categories of bugs at compile time\n**Performance**: Zero-cost abstractions with manual memory management speed\n**Concurrency**: Fearless parallelism with the ownership system\n**Reliability**: \"If it compiles, it works\" philosophy",
        {
            "background_color": "#fff5f5",
            "border_left": "6px solid #e74c3c",
            "padding": "40px",
            "margin": "30px auto",
            "max_width": "800px",
            "border_radius": "12px",
            "box_shadow": "0 4px 12px rgba(0,0,0,0.1)"
        }
    ))
    
    components.append(create_page_component(
        "Card",
        "⚡ Axum - High-Performance Backend\n\n**Async-First Design**: Built on Tokio for maximum concurrency\n**Type-Safe Routing**: Request/response validation at compile time\n**Middleware Ecosystem**: Composable request/response processing\n**Performance**: 100,000+ requests/second on commodity hardware",
        {
            "background_color": "#f0f8ff",
            "border_left": "6px solid #3498db",
            "padding": "40px",
            "margin": "30px auto",
            "max_width": "800px",
            "border_radius": "12px",
            "box_shadow": "0 4px 12px rgba(0,0,0,0.1)"
        }
    ))
    
    components.append(create_page_component(
        "Card",
        "🎨 Yew - WebAssembly Frontend\n\n**React-like API**: Familiar component-based development\n**WebAssembly Performance**: Near-native speed in the browser\n**Type Safety**: Full compile-time checking for UI components\n**Small Bundle Size**: Optimized WASM output",
        {
            "background_color": "#f0fff0",
            "border_left": "6px solid #27ae60",
            "padding": "40px",
            "margin": "30px auto",
            "max_width": "800px",
            "border_radius": "12px",
            "box_shadow": "0 4px 12px rgba(0,0,0,0.1)"
        }
    ))
    
    # Performance Comparison
    components.append(create_page_component(
        "Heading",
        "📊 Performance Revolution",
        {
            "font_size": "42px",
            "margin": "80px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    components.append(create_page_component(
        "Card",
        "Benchmark Results\n\n**Performance Improvements:**\n• 3x faster page load times vs React/Node.js\n• 10x lower memory usage vs Java applications\n• 5x higher concurrent user capacity\n\n**Development Productivity:**\n• 50% fewer production bugs\n• 2x faster feature development\n• 90% reduction in runtime debugging",
        {
            "background_color": "#fffacd",
            "border_radius": "16px",
            "padding": "40px",
            "margin": "40px auto",
            "max_width": "700px",
            "box_shadow": "0 8px 24px rgba(0,0,0,0.15)",
            "text_align": "center",
            "font_size": "18px"
        }
    ))
    
    # CTA
    components.append(create_page_component(
        "Button",
        "Explore CMS Features",
        {
            "background_color": "#e74c3c",
            "text_color": "white",
            "padding": "18px 36px",
            "border_radius": "12px",
            "font_size": "20px",
            "font_weight": "bold",
            "margin": "60px auto",
            "text_align": "center",
            "box_shadow": "0 4px 12px rgba(231,76,60,0.3)"
        },
        {
            "button_text": "Explore CMS Features",
            "button_url": "/page/features-capabilities",
            "button_target": "_self",
            "button_variant": "primary"
        }
    ))
    
    return components

def get_improved_features_components() -> List[Dict]:
    """Generate improved components for the Features & Capabilities page."""
    components = []
    
    # Hero Section
    components.append(create_page_component(
        "Hero",
        "🎯 Features & Capabilities",
        {
            "background_color": "linear-gradient(135deg, #74b9ff 0%, #0984e3 100%)",
            "text_color": "white",
            "padding": "100px 40px",
            "text_align": "center",
            "font_size": "56px",
            "font_weight": "bold",
            "margin": "0px",
            "border_radius": "0px"
        }
    ))
    
    # Subtitle
    components.append(create_page_component(
        "Subheading",
        "Comprehensive Content Management with Cutting-Edge Technology",
        {
            "text_align": "center",
            "font_size": "24px",
            "color": "#555",
            "margin": "40px auto",
            "max_width": "700px",
            "line_height": "1.5"
        }
    ))
    
    # Frontend Excellence Section
    components.append(create_page_component(
        "Heading",
        "🎨 Frontend Excellence",
        {
            "font_size": "42px",
            "margin": "80px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # Three Column Layout for Frontend Features
    components.append(create_page_component(
        "ThreeColumn",
        "🎨 **Visual Page Builder**\n• Drag-and-Drop Interface\n• Reusable Components\n• Live Preview\n• Mobile Optimization\n• Component Library\n\n📊 **Admin Dashboard**\n• Real-Time Analytics\n• Performance Monitoring\n• Content Insights\n• Quick Actions\n• Mobile-Responsive\n\n✍️ **Content Management**\n• Rich Markdown Editor\n• WYSIWYG Option\n• Media Integration\n• Version Control\n• Content Templates",
        {
            "margin": "40px 0px",
            "padding": "20px"
        }
    ))
    
    # Backend Power Section
    components.append(create_page_component(
        "Heading",
        "⚡ Backend Power",
        {
            "font_size": "42px",
            "margin": "80px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    # Backend Features as Cards
    components.append(create_page_component(
        "Card",
        "🚀 High-Performance API\n\n• **Sub-millisecond Response**: Axum-powered ultra-fast endpoints\n• **RESTful Architecture**: Clean, predictable API design\n• **Real-Time Updates**: WebSocket support for live content\n• **Rate Limiting**: Built-in abuse protection\n• **GraphQL Ready**: Flexible query capabilities",
        {
            "background_color": "#f8f9fa",
            "border_radius": "16px",
            "padding": "40px",
            "margin": "30px auto",
            "max_width": "800px",
            "box_shadow": "0 6px 20px rgba(0,0,0,0.1)"
        }
    ))
    
    components.append(create_page_component(
        "Card",
        "🔐 Security & Authentication\n\n• **Session-Based Auth**: Secure, stateful authentication\n• **Role-Based Access**: Granular permission system\n• **bcrypt Encryption**: Industry-standard password hashing\n• **CSRF Protection**: Built-in security measures\n• **Two-Factor Auth**: Enhanced security options",
        {
            "background_color": "#f8f9fa",
            "border_radius": "16px",
            "padding": "40px",
            "margin": "30px auto",
            "max_width": "800px",
            "box_shadow": "0 6px 20px rgba(0,0,0,0.1)"
        }
    ))
    
    # Developer Features
    components.append(create_page_component(
        "Heading",
        "🛠️ Developer Experience",
        {
            "font_size": "42px",
            "margin": "80px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    components.append(create_page_component(
        "Card",
        "🦀 Modern Development Stack\n\n• **Rust Everywhere**: Single language across full stack\n• **Type Safety**: Compile-time error prevention\n• **Cargo Integration**: Excellent package management\n• **Built-in Testing**: Comprehensive test framework\n• **Auto Documentation**: Self-documenting code\n• **Hot Reload**: Fast development iteration",
        {
            "background_color": "#fffacd",
            "border_radius": "16px",
            "padding": "40px",
            "margin": "40px auto",
            "max_width": "700px",
            "box_shadow": "0 8px 24px rgba(0,0,0,0.15)"
        }
    ))
    
    # CTA Section
    components.append(create_page_component(
        "Heading",
        "🚀 Ready to Get Started?",
        {
            "font_size": "42px",
            "margin": "80px auto 40px auto",
            "color": "#2c3e50",
            "text_align": "center",
            "font_weight": "bold"
        }
    ))
    
    components.append(create_page_component(
        "TwoColumn",
        "**Learn the Technology**\n\nDive deep into the RAYDT Stack and discover how Rust revolutionizes web development.\n\n**About the CMS**\n\nLearn more about My Rust CMS and why it's the future of content management.",
        {
            "margin": "40px 0px",
            "padding": "20px"
        }
    ))
    
    components.append(create_page_component(
        "Button",
        "Learn About RAYDT Stack",
        {
            "background_color": "#0984e3",
            "text_color": "white",
            "padding": "18px 36px",
            "border_radius": "12px",
            "font_size": "20px",
            "font_weight": "bold",
            "margin": "30px auto",
            "text_align": "center",
            "box_shadow": "0 4px 12px rgba(9,132,227,0.3)"
        },
        {
            "button_text": "Learn About RAYDT Stack",
            "button_url": "/page/raydt-stack-technology",
            "button_target": "_self",
            "button_variant": "primary"
        }
    ))
    
    components.append(create_page_component(
        "Button",
        "About My Rust CMS",
        {
            "background_color": "#00b894",
            "text_color": "white",
            "padding": "18px 36px",
            "border_radius": "12px",
            "font_size": "20px",
            "font_weight": "bold",
            "margin": "30px auto",
            "text_align": "center",
            "box_shadow": "0 4px 12px rgba(0,184,148,0.3)"
        },
        {
            "button_text": "About My Rust CMS",
            "button_url": "/page/about-my-rust-cms",
            "button_target": "_self",
            "button_variant": "secondary"
        }
    ))
    
    return components

def create_page(token: str, page_data: Dict[str, Any]) -> bool:
    """Create a page using the API."""
    headers = {
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json"
    }
    
    try:
        response = requests.post(PAGES_URL, headers=headers, json=page_data)
        if response.status_code == 201:
            print(f"✅ Created page: {page_data['title']}")
            return True
        else:
            print(f"❌ Failed to create page '{page_data['title']}': {response.status_code}")
            print(f"   Response: {response.text}")
            return False
    except Exception as e:
        print(f"❌ Error creating page '{page_data['title']}': {e}")
        return False

def main():
    """Main function to regenerate promotional pages."""
    print("🔄 My Rust CMS - Promotional Pages Regeneration")
    print("=" * 60)
    
    # Authenticate
    print("🔐 Authenticating...")
    token = authenticate()
    if not token:
        print("❌ Failed to authenticate. Please check credentials and server status.")
        return
    
    print("✅ Authentication successful!")
    
    # Delete old promotional pages
    old_page_ids = [38, 39, 40]  # IDs found earlier
    print(f"\n🗑️  Deleting {len(old_page_ids)} old promotional pages...")
    
    deleted_count = 0
    for page_id in old_page_ids:
        if delete_page(token, page_id):
            deleted_count += 1
        time.sleep(0.5)  # Brief pause between deletions
    
    print(f"✅ Deleted {deleted_count}/{len(old_page_ids)} old pages")
    
    # Create new promotional pages with improved components
    new_pages = [
        {
            "title": "About My Rust CMS",
            "slug": "about-my-rust-cms",
            "components": get_improved_about_cms_components()
        },
        {
            "title": "RAYDT Stack Technology",
            "slug": "raydt-stack-technology",
            "components": get_improved_raydt_stack_components()
        },
        {
            "title": "Features & Capabilities",
            "slug": "features-capabilities",
            "components": get_improved_features_components()
        }
    ]
    
    print(f"\n🆕 Creating {len(new_pages)} new promotional pages with improved components...")
    created_count = 0
    
    for page_data in new_pages:
        print(f"\n📝 Creating: {page_data['title']}")
        
        # Convert components to JSON string
        content_json = json.dumps(page_data["components"])
        
        api_page_data = {
            "title": page_data["title"],
            "slug": page_data["slug"],
            "content": content_json,
            "status": "published"
        }
        
        if create_page(token, api_page_data):
            created_count += 1
        time.sleep(1)  # Brief pause between creations
    
    # Summary
    print("\n" + "=" * 60)
    print(f"🎉 Regeneration Complete!")
    print(f"   • Deleted: {deleted_count} old pages")
    print(f"   • Created: {created_count} new pages")
    
    if created_count > 0:
        print(f"\n📋 New Promotional Pages:")
        for page in new_pages[:created_count]:
            print(f"   • {page['title']} → /page/{page['slug']}")
        
        print(f"\n🌐 Visit your improved pages at:")
        for page in new_pages[:created_count]:
            print(f"   http://localhost:8080/page/{page['slug']}")
        
        print(f"\n✨ New Features:")
        print(f"   • Improved Hero sections with dramatic gradients")
        print(f"   • Professional Card components with shadows")
        print(f"   • Functional Button navigation between pages")
        print(f"   • Enhanced Typography and spacing")
        print(f"   • Responsive multi-column layouts")
        print(f"   • Better visual hierarchy and readability")
    
    print("\n🚀 Your promotional pages now showcase the full power of the improved component system!")

if __name__ == "__main__":
    main()