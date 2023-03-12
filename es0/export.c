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

void export(ExportData *data, int n, FILE *fp) {
    for(int i=0; i<n; i++)
        fwrite(&data[i], sizeof(ExportData), 1, fp);
}

int main() {
    ExportData export_data_arr[100];
    srand(time(NULL)); // seed the random number generator with current time
    FILE *fp;

    char *strings[100] = {
            "Artificial heart",
            "Space exploration",
            "Quantum computing",
            "Renewable energy",
            "Virtual reality",
            "AI",
            "Robotics technology",
            "Neuroscience research",
            "Genetic engineering",
            "3D printing",
            "Smart cities",
            "Blockchain technology",
            "Augmented reality",
            "Nanotechnology",
            "Clean water",
            "Green energy",
            "Autonomous vehicles",
            "Wireless power",
            "Smart homes",
            "Advanced materials",
    };

    if((fp = fopen("exported_data.bin", "wb+")) == NULL) {
        printf("Error opening file \"exported_data.bin\".");
        exit(-1);
    }

    for (int i = 0; i < 100; i++) {
        int type = rand() % 3 + 1; // generate random type value between 1 and 3
        ExportData export_data;
        export_data.type = type;

        switch (type) {
            case 1:
                export_data.val.type = type;
                export_data.val.val = (float) rand() / (RAND_MAX * 100);
                export_data.val.timestamp = time(NULL);
                break;
            case 2:
                export_data.mvals.type = type;
                export_data.mvals.timestamp = time(NULL);
                for (int j = 0; j < 10; j++) {
                    export_data.mvals.val[j] = (float) (rand() / RAND_MAX * 100);
                }
                break;
            case 3:
                export_data.messages.type = type;
                char *message = strings[(int) (rand() % 20)];
                int len = strlen(message);
                strncpy(export_data.messages.message, strings[(int) (rand() % 20)], len);
                export_data.messages.message[len] = '\0';
                break;
        }

        export_data_arr[i] = export_data;
    }

    export(export_data_arr, 100, fp);

    fclose(fp);
    exit(0);
}
