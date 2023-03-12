#include <stdio.h>
#include <time.h>

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

    printf("sizeof(ValueStruct)   = %2lu\n", sizeof(ValueStruct));
    printf("sizeof(MValueStruct)  = %2lu\n", sizeof(MValueStruct));
    printf("sizeof(MessageStruct) = %2lu\n", sizeof(MessageStruct));
    printf("sizeof(ExportData)    = %2lu\n", sizeof(ExportData));

    ExportData exportData[100];
    for(int i=0; i<100; i++) {
        if((fread(&exportData[i], sizeof(ExportData), 1, fp)) < 0) {
            printf("Error reading from file...");
            return -2;
        }
        switch(exportData[i].type) {
            case 1:     printf("%02d: ValueStruct(%d, %f, %ld)\n", i+1, exportData[i].val.type, exportData[i].val.val, exportData[i].val.timestamp);
                        break;
            case 2:     printf("%02d: MValueStruct(%d, 0x%x, %ld)\n", i+1, exportData[i].mvals.type, &(exportData[i].mvals.val), exportData[i].mvals.timestamp);
                        break;
            case 3:     printf("%02d: MessageStruct(%d, \"%s\")\n", i+1, exportData[i].messages.type, exportData[i].messages.message);
                        break;
            default:    printf("Error reading element %d\n", i+1);
                        return -3;
        }
    }

    printf("Legacy data has been stored successfully!\n");
    return 0;
}
