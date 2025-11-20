/*
 * Copied from:
 * https://github.com/godot-rust/gdext/blob/master/godot-codegen/src/special_cases/special_cases.rs
 *
 * These functions are pulled from gdext since they are not accessible normally.
 * I've made an Issue to expose the behavior of `is_godot_type_deleted`, so maybe I can get rid of this eventually.
 * https://github.com/godot-rust/gdext/issues/1409
 *
 * There may also be a better way to do this?
 */

#![allow(unexpected_cfgs)]

/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[rustfmt::skip]
pub fn is_godot_type_deleted(godot_ty: &str) -> bool {
    // Note: parameter can be a class or builtin name, but also something like "enum::AESContext.Mode".

    // Exclude experimental APIs unless opted-in.
    if !cfg!(feature = "experimental-godot-api") && is_class_experimental(godot_ty) {
        return true;
    }

    // OpenXR has not been available for "macos" before 4.2 (now no longer supported by godot-rust).
    // See e.g. https://github.com/GodotVR/godot-xr-tools/issues/479.
    // OpenXR is also not available on iOS and Web: https://github.com/godotengine/godot/blob/13ba673c42951fd7cfa6fd8a7f25ede7e9ad92bb/modules/openxr/config.py#L2
    // Do not hardcode a list of OpenXR classes, as more may be added in future Godot versions; instead use prefix.
    if godot_ty.starts_with("OpenXR") {
        let target_os = std::env::var("CARGO_CFG_TARGET_OS");
        match target_os.as_deref() {
            Ok("ios") | Ok("emscripten") => return true,
            _ => {}
        }
    }

    // cfg!(target_os = "...") are relatively new and need more testing. If causing problems, revert to `true` (deleted) for now.
    // TODO(v0.5): for doc generation, consider moving the target-filters to the generated code, so that API docs still show the classes.
    match godot_ty {
        // Only on Android.
        | "JavaClass"
        | "JavaClassWrapper"
        | "JavaObject"
        | "JNISingleton"
        => !cfg!(target_os = "android"),

        // Only on Wasm.
        | "JavaScriptBridge"
        | "JavaScriptObject"
        => !cfg!(target_os = "emscripten"),

        // Thread APIs.
        | "Thread"
        | "Mutex"
        | "Semaphore"
        => true,

        // Reintroduced in 4.3: https://github.com/godotengine/godot/pull/80214
        | "UniformSetCacheRD"
        => cfg!(before_api = "4.3"),

        _ => false
    }

    // Older special cases:
    // * ThemeDB was loaded lazily; from 4.2 it loads at the Scene level: https://github.com/godotengine/godot/pull/81305
    // * Internal classes were accidentally exposed < 4.2: https://github.com/godotengine/godot/pull/80852: FramebufferCacheRD,
    //   GDScriptEditorTranslationParserPlugin, GDScriptNativeClass, GLTFDocumentExtensionPhysics, GLTFDocumentExtensionTextureWebP,
    //   GodotPhysicsServer2D, GodotPhysicsServer3D, IPUnix, MovieWriterMJPEG, MovieWriterPNGWAV, ResourceFormatImporterSaver
}

#[rustfmt::skip]
pub fn is_class_experimental(godot_class_name: &str) -> bool {
    // Note: parameter can be a class or builtin name, but also something like "enum::AESContext.Mode".

    // These classes are currently hardcoded, but the information is available in Godot's doc/classes directory.
    // The XML file contains a property <class name="NavigationMesh" ... experimental="">.

    // Last update: 2024-09-15; Godot rev 6681f2563b99e14929a8acb27f4908fece398ef1.
    match godot_class_name {
        | "AudioSample"
        | "AudioSamplePlayback"
        | "Compositor"
        | "CompositorEffect"
        | "GraphEdit"
        | "GraphElement"
        | "GraphFrame"
        | "GraphNode"
        | "NavigationAgent2D"
        | "NavigationAgent3D"
        | "NavigationLink2D"
        | "NavigationLink3D"
        | "NavigationMesh"
        | "NavigationMeshSourceGeometryData2D"
        | "NavigationMeshSourceGeometryData3D"
        | "NavigationObstacle2D"
        | "NavigationObstacle3D"
        | "NavigationPathQueryParameters2D"
        | "NavigationPathQueryParameters3D"
        | "NavigationPathQueryResult2D"
        | "NavigationPathQueryResult3D"
        | "NavigationPolygon"
        | "NavigationRegion2D"
        | "NavigationRegion3D"
        | "NavigationServer2D"
        | "NavigationServer3D"
        | "Parallax2D"
        | "SkeletonModification2D"
        | "SkeletonModification2DCCDIK"
        | "SkeletonModification2DFABRIK"
        | "SkeletonModification2DJiggle"
        | "SkeletonModification2DLookAt"
        | "SkeletonModification2DPhysicalBones"
        | "SkeletonModification2DStackHolder"
        | "SkeletonModification2DTwoBoneIK"
        | "SkeletonModificationStack2D"
        | "StreamPeerGZIP"
        | "XRBodyModifier3D"
        | "XRBodyTracker"
        | "XRFaceModifier3D"
        | "XRFaceTracker"

        => true, _ => false
    }
}
