#include <iostream>
#include <SDL2/SDL.h>
#include <epoxy/gl.h>
#include <cstdlib>
#include "imgui.h"
#include "imgui_impl_sdl.h"
#include "imgui_impl_opengl3.h"

static inline void setup_gl() {
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_FLAGS, 0);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, SDL_GL_CONTEXT_PROFILE_CORE);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
    SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
    SDL_GL_SetAttribute(SDL_GL_DOUBLEBUFFER, 1);
    SDL_GL_SetAttribute(SDL_GL_DEPTH_SIZE, 24);
    SDL_GL_SetAttribute(SDL_GL_STENCIL_SIZE, 8);
}

static inline void render_window(SDL_Window* const wnd) {
    auto show = true;
    int width, height;
    SDL_GetWindowSize(wnd, &width, &height);
    ImGui::SetNextWindowPos(ImVec2(.0f, 0.f), ImGuiCond_Always);
    ImGui::SetNextWindowSize(ImVec2(width + 2, height + 2), ImGuiCond_Always);
    ImGui::Begin("main", &show, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove | ImGuiWindowFlags_NoCollapse);
    ImGui::Text("hiya");
    ImGui::Text("avg %.3f ms/frame", 1000.0f / ImGui::GetIO().Framerate);
    ImGui::End();
}

int main(int argc, char **argv) {
    if (std::getenv("WAYLAND_DISPLAY")) {
        setenv("SDL_VIDEODRIVER", "wayland", false);
    }

    if (SDL_Init(SDL_INIT_VIDEO) != 0) {
        std::cerr << "couldn't init sdl" << std::endl;

        return 1;
    }

    setup_gl();
    const auto flags = SDL_WINDOW_OPENGL | SDL_WINDOW_SHOWN | SDL_WINDOW_RESIZABLE;
    auto* wnd = SDL_CreateWindow("lmmp", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 200, 200, flags);
    if (!wnd) {
        std::cerr << "no window: " << SDL_GetError() << std::endl;

        return 1;
    }
    auto glctx = SDL_GL_CreateContext(wnd);
    SDL_GL_MakeCurrent(wnd, glctx);
    SDL_GL_SetSwapInterval(1);
    SDL_ShowWindow(wnd);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    auto& io = ImGui::GetIO();

    ImGui::StyleColorsDark();

    ImGui_ImplSDL2_InitForOpenGL(wnd, glctx);
    ImGui_ImplOpenGL3_Init("#version 130");

    SDL_Event ev;

    bool done = false;
    while (!done) {
        while (SDL_PollEvent(&ev) == 1) {
            ImGui_ImplSDL2_ProcessEvent(&ev);
            switch (ev.type) {
            case SDL_QUIT:
                done = true;
                goto quit;
            }
        }

        ImGui_ImplOpenGL3_NewFrame();
        ImGui_ImplSDL2_NewFrame(wnd);
        ImGui::NewFrame();

        auto show = true;

        int width, height;
        SDL_GetWindowSize(wnd, &width, &height);

        ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(2.f, 2.f));
        ImGui::PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2(0.f, 0.f));
        ImGui::PushStyleVar(ImGuiStyleVar_WindowMinSize, ImVec2(width, 0.f));
        ImGui::SetNextWindowPos(ImVec2(.0f, 0.f), ImGuiCond_Always);
        ImGui::SetNextWindowSize(ImVec2(0, 0));
        ImGui::Begin("toolbar", &show, ImGuiWindowFlags_NoDecoration);
        ImGui::Text("lmmp"); ImGui::SameLine();
        ImGui::Button("play/pause"); ImGui::SameLine();
        ImGui::Button("next"); ImGui::SameLine();
        ImGui::Button("previous"); ImGui::SameLine();
        auto toolbar_height = ImGui::GetWindowHeight();
        ImGui::End();
        ImGui::PopStyleVar();
        ImGui::PopStyleVar();
        ImGui::PopStyleVar();

        auto split_height = (height - toolbar_height * 2) / 2;

        ImGui::SetNextWindowPos(ImVec2(0.f, toolbar_height), ImGuiCond_Always);
        ImGui::SetNextWindowSize(ImVec2(split_height, split_height), ImGuiCond_Always);
        ImGui::Begin("art", &show, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_NoNav);
        ImGui::Text("art");
        ImGui::End();

        ImGui::SetNextWindowPos(ImVec2(split_height, toolbar_height), ImGuiCond_Always);
        ImGui::SetNextWindowSize(ImVec2(width, split_height), ImGuiCond_Always);
        ImGui::Begin("library", &show, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_NoNav);
        ImGui::Text("library");
        ImGui::End();

        ImGui::SetNextWindowPos(ImVec2(0.f, toolbar_height + split_height), ImGuiCond_Always);
        ImGui::SetNextWindowSize(ImVec2(width, split_height), ImGuiCond_Always);
        ImGui::Begin("playlist", &show, ImGuiWindowFlags_NoTitleBar | ImGuiWindowFlags_NoDecoration | ImGuiWindowFlags_NoInputs | ImGuiWindowFlags_NoNav);
        ImGui::Text("playlist");
        ImGui::End();

        ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(2.f, 2.f));
        ImGui::PushStyleVar(ImGuiStyleVar_FramePadding, ImVec2(0.f, 0.f));
        ImGui::PushStyleVar(ImGuiStyleVar_WindowMinSize, ImVec2(width, 0.f));
        ImGui::SetNextWindowPos(ImVec2(0.f, height - toolbar_height), ImGuiCond_Always);
        ImGui::SetNextWindowSize(ImVec2(0, 0));
        ImGui::Begin("statusbar", &show, ImGuiWindowFlags_NoDecoration);
        ImGui::Text("statusbar"); ImGui::SameLine();
        ImGui::End();
        ImGui::PopStyleVar();
        ImGui::PopStyleVar();
        ImGui::PopStyleVar();

        goto render;

render:
        ImGui::Render();
        glViewport(0, 0, (int)io.DisplaySize.x, (int)io.DisplaySize.y);
        /* glClearColor(0.0f, 0.0f, 0.0f, 0.0f); */
        /* glClear(GL_COLOR_BUFFER_BIT); */
        ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
        SDL_GL_SwapWindow(wnd);
        SDL_WaitEvent(NULL);
    }

quit:
    ImGui_ImplOpenGL3_Shutdown();
    ImGui_ImplSDL2_Shutdown();
    ImGui::DestroyContext();
    SDL_DestroyWindow(wnd);
    SDL_Quit();

    return 0;
}
