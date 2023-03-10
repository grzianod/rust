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
    char message[21]; // stringa null terminated lung max 20
} MessageStruct;

typedef struct {
        int type;
        union {
            ValueStruct val;
            MValueStruct mvals;
            MessageStruct messages; };
} ExportData;

int main() {
    ExportData export_data_arr[100];
    srand(time(NULL)); // seed the random number generator with current time

    FILE *fp;

    if((fp = fopen("export_data.bin", "wb+")) == NULL) {
        printf("Error opening file \"data.bin\".");
        exit(-1);
    }

    for (int i = 0; i < 100; i++) {
        int type = rand() % 3 + 1; // generate random type value between 1 and 3
        ExportData export_data;
        export_data.type = type;

        switch (type) {
            case 1:
                export_data.val.type = type;
                export_data.val.val = (float) rand() / RAND_MAX * 100;
                export_data.val.timestamp = time(NULL);
                break;
            case 2:
                export_data.mvals.type = type;
                export_data.mvals.timestamp = time(NULL);
                for (int j = 0; j < 10; j++) {
                    export_data.mvals.val[j] = (float) rand() / RAND_MAX * 100;
                }
                break;
            case 3:
                export_data.messages.type = type;
                char message[21];
                int len = rand() % 20 + 1; // generate random message length between 1 and 20
                for (int j = 0; j < len; j++) {
                    message[j] = 'a' + rand() % 26; // generate random lowercase letter
                }
                message[len] = '\0'; // add null terminator to message string
                strncpy(export_data.messages.message, message, sizeof(export_data.messages.message));
                break;
        }

        export_data_arr[i] = export_data;
    }

    for(int i=0; i<100; i++)
            fwrite(&export_data_arr[i], sizeof(ExportData), 1, fp);
    fclose(fp);
    exit(0);
}
