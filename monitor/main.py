import cv2
import mediapipe as mp
from mediapipe.tasks import python
from mediapipe.tasks.python import vision
import csv

MODEL_PATH = "models/pose_landmarker_lite.task"

base_options = python.BaseOptions(model_asset_path=MODEL_PATH)
options = vision.PoseLandmarkerOptions(
    base_options=base_options, running_mode=vision.RunningMode.VIDEO, num_poses=1
)
pose_landmarker = vision.PoseLandmarker.create_from_options(options)

cap = cv2.VideoCapture(0)
frame_idx = 0

csv_file = open("poses.csv", "w", newline="")
csv_writer = csv.writer(csv_file)

header = ["frame"]
for i in range(33):
    header.extend([f"x{i}", f"y{i}", f"z{i}", f"v{i}"])
csv_writer.writerow(header)

while cap.isOpened():
    success, frame = cap.read()
    if not success:
        break

    frame_idx += 1

    rgb_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)

    mp_image = mp.Image(image_format=mp.ImageFormat.SRGB, data=rgb_frame)

    result = pose_landmarker.detect_for_video(
        mp_image,
        timestamp_ms=frame_idx * 33,
    )

    if result.pose_landmarks:
        for pose_landmarks in result.pose_landmarks:
            row = [frame_idx]

            for landmark in pose_landmarks:
                row.extend([landmark.x, landmark.y, landmark.z, landmark.visibility])
            csv_writer.writerow(row)

            for landmark in pose_landmarks:
                h, w, _ = frame.shape
                cx, cy = int(landmark.x * w), int(landmark.y * h)
                cv2.circle(frame, (cx, cy), 3, (0, 255, 0), -1)

    cv2.imshow("MediaPipe Pose (Tasks API)", frame)

    if cv2.waitKey(100) & 0xFF == 27:  # ESC
        break


cap.release()
cv2.destroyAllWindows()
csv_file.close()
pose_landmarker.close()
