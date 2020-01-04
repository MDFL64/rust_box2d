extern crate cc;
extern crate cmake;

fn main() {
    /*println!("cargo:rustc-link-lib=static=Box2D");
    if let Some(path) = std::env::var("BOX2D_LIB_DIR").ok() {
        println!("cargo:rustc-link-search=native={}", path);
    } else {
        let box2d_install_prefix = cmake::Config::new("Box2D/Box2D")
            .env("CC", "clang")
            .env("CXX", "clang++")
            .define("BOX2D_BUILD_STATIC", "ON")
            .define("BOX2D_INSTALL", "ON")
            .define("BOX2D_BUILD_SHARED", "OFF")
            .define("BOX2D_BUILD_EXAMPLES", "OFF")
            .define("BOX2D_INSTALL_DOC", "OFF")
            .build();
        println!("cargo:rustc-link-search=native={}/lib", box2d_install_prefix.display());
    };*/

    cc::Build::new()
        .cpp(true)

        .warnings(false)
        //.flag("-std=c++14")
        .flag("-Wno-everything")
        //.flag("-xc++")
        .flag("-fno-rtti")
        .cpp_link_stdlib(None)
        //.cpp_set_stdlib("c++")
        .archiver("llvm-ar")

        .include("fake_sys_headers")
        .include("Box2D/Box2D")
        .file("frontend/lib.cpp")

        .file("Box2D/Box2D/Box2D/Common/b2BlockAllocator.cpp")
        .file("Box2D/Box2D/Box2D/Common/b2Math.cpp")
        .file("Box2D/Box2D/Box2D/Common/b2Settings.cpp")
        .file("Box2D/Box2D/Box2D/Common/b2StackAllocator.cpp")
        .file("Box2D/Box2D/Box2D/Common/b2Timer.cpp")
        .file("Box2D/Box2D/Box2D/Common/b2Draw.cpp")

        .file("Box2D/Box2D/Box2D/Collision/b2BroadPhase.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2CollideCircle.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2CollideEdge.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2CollidePolygon.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2Collision.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2Distance.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2DynamicTree.cpp")
        .file("Box2D/Box2D/Box2D/Collision/b2TimeOfImpact.cpp")

        .file("Box2D/Box2D/Box2D/Collision/Shapes/b2ChainShape.cpp")
        .file("Box2D/Box2D/Box2D/Collision/Shapes/b2CircleShape.cpp")
        .file("Box2D/Box2D/Box2D/Collision/Shapes/b2EdgeShape.cpp")
        .file("Box2D/Box2D/Box2D/Collision/Shapes/b2PolygonShape.cpp")

        .file("Box2D/Box2D/Box2D/Dynamics/b2Body.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/b2ContactManager.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/b2Fixture.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/b2Island.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/b2World.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/b2WorldCallbacks.cpp")
        
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2ChainAndCircleContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2ChainAndPolygonContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2CircleContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2Contact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2ContactSolver.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2EdgeAndCircleContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2EdgeAndPolygonContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2EdgeAndPolygonContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2PolygonAndCircleContact.cpp")
        .file("Box2D/Box2D/Box2D/Dynamics/Contacts/b2PolygonContact.cpp")

        // TODO JOINTS

        .compile("cbox2d");
}
