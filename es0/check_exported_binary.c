#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;
    float val[10];
    long timestamp;
} MValueStruct;

typedef struct {
    int type;
    char message[21];
} MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages; };
} ExportData;

int main() {
    FILE *fp;
    if((fp = fopen("exported_data.bin", "rb")) == NULL) {
        printf("Error opening file\n");
        return -1;
    }

    ExportData exportData[100];
    for(int i=0; i<100; i++) {
        fread(&exportData[i], sizeof(ExportData), 1, fp);
        switch(exportData[i].type) {
            case 1:     printf("%02d: ValueStruct(%d, %f, %ld)\n", i+1, exportData[i].val.type, exportData[i].val.val, exportData[i].val.timestamp);
                        break;
            case 2:     printf("%02d: MValueStruct(%d, 0x%x, %ld)\n", i+1, exportData[i].mvals.type, &(exportData[i].mvals.val), exportData[i].mvals.timestamp);
                        break;
            case 3:     printf("%02d: MessageStruct(%d, \"%s\")\n", i+1, exportData[i].messages.type, exportData[i].messages.message);
                        break;
            default: printf("This will cause PANIC in Rust!");
        }
    }
    return 0;
}
