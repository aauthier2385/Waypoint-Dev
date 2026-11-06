"""Catch the falling stars. Move with the left and right arrow keys."""
import random

import pygame

WIDTH, HEIGHT = 640, 480
PLAYER_SPEED = 7  # Try making this bigger.
STAR_SPEED = 4


def new_star():
    return pygame.Rect(random.randint(0, WIDTH - 20), -20, 20, 20)


def main():
    pygame.init()
    screen = pygame.display.set_mode((WIDTH, HEIGHT))
    pygame.display.set_caption("Star Catcher")
    clock = pygame.time.Clock()
    font = pygame.font.Font(None, 36)
    player = pygame.Rect(WIDTH // 2 - 40, HEIGHT - 40, 80, 16)
    star = new_star()
    score = 0
    running = True

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

        keys = pygame.key.get_pressed()
        if keys[pygame.K_LEFT]:
            player.x -= PLAYER_SPEED
        if keys[pygame.K_RIGHT]:
            player.x += PLAYER_SPEED
        player.clamp_ip(screen.get_rect())

        star.y += STAR_SPEED
        if star.colliderect(player):
            score += 1
            star = new_star()
        elif star.top > HEIGHT:
            score = max(0, score - 1)
            star = new_star()

        screen.fill((24, 32, 44))
        pygame.draw.rect(screen, (255, 209, 102), star, border_radius=6)
        pygame.draw.rect(screen, (92, 200, 170), player, border_radius=6)
        screen.blit(font.render(f"Score: {score}", True, (240, 240, 240)), (16, 12))
        pygame.display.flip()
        clock.tick(60)

    pygame.quit()


if __name__ == "__main__":
    main()
