from PIL import Image
from time import time

def pegar_lados(pos:tuple, tamX, tamY):

    saida = []

    if ((pos[0] - 1) >= 0 and (pos[0] - 1) <= tamX):
        saida.append((pos[0] - 1, pos[1]))

    if ((pos[0] + 1) >= 0 and (pos[0] + 1) <= tamX):
        saida.append((pos[0] + 1, pos[1]))

    if ((pos[1] - 1) >= 0 and (pos[1] - 1) <= tamY):
        saida.append((pos[0], pos[1] - 1))

    if ((pos[1] + 1) >= 0 and (pos[1] + 1) <= tamY):
        saida.append((pos[0], pos[1] + 1))  

    return saida


##################################################################################################################
#                                                                                                                #
#                                    Declaração de variaveis e constantes                                        #
#                                                                                                                #
##################################################################################################################

cor_esperada = (255, 255, 255, 255)
cor_alvo = (255, 255, 255, 0)
img = Image.open(input("Caminho para a imagem: ")).convert("RGBA")
pixels = img.load()
imgX, imgY = img.size
imgX, imgY = imgX-1, imgY-1

checar = []

##################################################################################################################
#                                                                                                                #
#                                             Programa principal                                                 #
#                                                                                                                #
##################################################################################################################

t1 = time()

###########################
#                         #
#     Remoção de Lados    #
#                         #
###########################


for A in range(imgX+1):

    if pixels[A, 0] == cor_esperada and (A, 0) not in checar:
        checar.append((A, 0))

    if pixels[A, imgY] == cor_esperada and (A, imgY) not in checar:
        checar.append((A, imgY))

for A in range(imgY+1):

    if pixels[0, A] == cor_esperada and (0, A) not in checar:
        checar.append((0, A))

    if pixels[imgX, A] == cor_esperada and (imgX, A) not in checar:
        checar.append((imgX, A))


###########################
#                         #
#      Processamento      #
#                         #
###########################


while len(checar) > 0:

    px = checar[0]

    if pixels[px[0], px[1]] == cor_alvo:
        checar.pop(0)
        continue

    for A in pegar_lados(px, imgX, imgY):

        if pixels[A[0], A[1]] == cor_esperada and (A[0], A[1]) not in checar:
            checar.append((A[0], A[1]))


    pixels[px[0], px[1]] = cor_alvo
    checar.pop(0)
    print(len(checar))

t2 = time()

img.save("saida.png")


print(f"tempo de execução = {t2-t1}")
