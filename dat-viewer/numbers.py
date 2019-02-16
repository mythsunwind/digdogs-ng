import sys

#
# Print addresses of tiles
#

for row in range(0, 20):
    for x in range(row*50, row*50+50):
        sys.stdout.write("%02x  " % x)
    print("\n")

